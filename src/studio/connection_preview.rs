//! Read-only, span-sized projection of planned ownership (never a router).
use crate::{canvas::View, theme::Pal};
use eframe::egui::{self, Rect, Stroke};
use zigzag::{design::Design, geom::P};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Portion {
    pub owner: usize,
    pub block: usize,
    pub run: usize,
    pub start: P,
    pub end: P,
    pub break_before: bool,
}

/// One item per ownership span, not per stitch. Invalid metadata fails closed.
pub fn portions(d: &Design) -> Option<Vec<Portion>> {
    if d.blocks.len() != d.stitch_shapes.len() {
        return None;
    }
    let mut out = Vec::new();
    let mut cut = false;
    let mut travel = std::collections::HashMap::new();
    for t in &d.travels {
        travel
            .entry((t.block, t.run, t.from.saturating_add(1), t.shape))
            .and_modify(|end: &mut usize| *end = (*end).max(t.to))
            .or_insert(t.to);
    }
    for (bi, (block, owners)) in d.blocks.iter().zip(&d.stitch_shapes).enumerate() {
        if block.runs.len() != owners.len() {
            return None;
        }
        cut |= bi > 0; // Color change or Stop, even when RGB is unchanged.
        for (ri, (run, spans)) in block.runs.iter().zip(owners).enumerate() {
            cut |= run.trim_before;
            if run.pts.is_empty() {
                if !spans.is_empty() {
                    return None;
                }
                continue;
            }
            if spans.first().map(|s| s.0) != Some(0) {
                return None;
            }
            for (si, &(start, owner)) in spans.iter().enumerate() {
                let end = spans.get(si + 1).map_or(run.pts.len(), |s| s.0);
                if start >= end || end > run.pts.len() || owner >= d.per_shape.len() {
                    return None;
                }
                let entry = travel
                    .get(&(bi, ri, start, owner))
                    .copied()
                    .unwrap_or(start);
                if entry >= end {
                    return None;
                }
                out.push(Portion {
                    owner,
                    block: bi,
                    run: ri,
                    start: run.pts[entry],
                    end: run.pts[end - 1],
                    break_before: cut,
                });
                cut = false;
            }
        }
    }
    Some(out)
}

#[derive(Debug, PartialEq)]
pub struct Preview {
    pub first: Portion,
    pub last: Portion,
    pub incoming: Option<(P, bool)>,
    pub outgoing: Option<(P, bool)>,
}

pub fn selected(parts: &[Portion], owner: usize) -> Option<Preview> {
    let first = parts.iter().position(|p| p.owner == owner)?;
    let last = parts.iter().rposition(|p| p.owner == owner)?;
    Some(Preview {
        first: parts[first],
        last: parts[last],
        incoming: first
            .checked_sub(1)
            .map(|i| (parts[i].end, parts[first].break_before)),
        outgoing: parts.get(last + 1).map(|p| (p.start, p.break_before)),
    })
}

pub fn paint(painter: &egui::Painter, rect: Rect, view: View, pal: Pal, p: &Preview) {
    let painter = painter.with_clip_rect(rect);
    let line = |a, b, cut, color| {
        let a = view.to_screen(rect, a);
        let b = view.to_screen(rect, b);
        // Bound tessellation at extreme zoom; egui clips the resulting segments.
        let count = ((a.distance(b) / 8.0).ceil() as usize).clamp(1, 2048);
        let color = if cut { pal.secondary } else { color };
        for i in 0..count {
            let t = i as f32 / count as f32;
            let u = (i as f32 + 0.55) / count as f32;
            painter.line_segment([a.lerp(b, t), a.lerp(b, u)], Stroke::new(3.5, pal.dock));
            painter.line_segment([a.lerp(b, t), a.lerp(b, u)], Stroke::new(1.5, color));
        }
        painter.circle_stroke(a, 3.0, Stroke::new(1.5, color));
        painter.circle_stroke(b, 3.0, Stroke::new(1.5, color));
    };
    if let Some((from, cut)) = p.incoming {
        line(from, p.first.start, cut, pal.accent);
    }
    if let Some((to, cut)) = p.outgoing {
        line(p.last.end, to, cut, pal.text);
    }
    for (at, label, color, offset) in [
        (p.first.start, "In", pal.accent, egui::vec2(7.0, -12.0)),
        (p.last.end, "Out", pal.text, egui::vec2(7.0, 4.0)),
    ] {
        let at = view.to_screen(rect, at);
        painter.circle_filled(at, 6.0, pal.dock);
        painter.circle_filled(at, 4.0, color);
        let galley = painter.layout_no_wrap(label.into(), egui::FontId::proportional(11.0), color);
        let label_rect = Rect::from_min_size(at + offset, galley.size());
        painter.rect_filled(label_rect.expand(2.0), 2.0, pal.dock);
        painter.galley(label_rect.min, galley, color);
    }
}

impl crate::app::Studio {
    pub(crate) fn connection_preview(&self) -> Option<Preview> {
        if !self.view_tools.selected_connections
            || self.playing
            || self.panning
            || self.mid_gesture()
            || self.route_drag.is_some()
            || self.angle_drag.is_some()
            || self.view_tools.marker_drag.is_some()
            || self.marquee.is_some()
            || self.sel_indices().len() != 1
        {
            return None;
        }
        let i = self.sel_index()?;
        let shape = self.doc.shapes.get(i)?;
        if shape.params.hidden()
            || shape.params.kind == Some(zigzag::params::Kind::Skip)
            || self.design.per_shape.get(i).copied().unwrap_or(0) == 0
        {
            return None;
        }
        selected(&portions(&self.design)?, i)
    }

    pub(crate) fn draw_connections(&self, painter: &egui::Painter, rect: Rect) {
        if painter.ctx().input(|i| i.pointer.is_decidedly_dragging()) {
            return;
        }
        if let Some(preview) = self.connection_preview() {
            paint(
                painter,
                rect,
                self.view,
                crate::theme::pal(painter.ctx()),
                &preview,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zigzag::design::{Block, Run, Travel, TravelHow};
    fn design() -> Design {
        Design {
            per_shape: vec![4, 2],
            blocks: vec![Block {
                color: [1, 2, 3],
                runs: vec![Run {
                    pts: (0..6).map(|x| P::new(x as f64, 0.0)).collect(),
                    trim_before: false,
                }],
            }],
            stitch_shapes: vec![vec![vec![(0, 0), (2, 1), (4, 0)]]],
            ..Default::default()
        }
    }
    #[test]
    fn repeated_first_last_and_adjacent_other() {
        let d = design();
        let parts = portions(&d).unwrap();
        let a = selected(&parts, 0).unwrap();
        assert_eq!((a.first.start.x, a.last.end.x), (0.0, 5.0));
        assert_eq!((a.incoming, a.outgoing), (None, None));
        let b = selected(&parts, 1).unwrap();
        assert_eq!(b.incoming, Some((P::new(1.0, 0.0), false)));
        assert_eq!(b.outgoing, Some((P::new(4.0, 0.0), false)));
        assert!(selected(&parts, 99).is_none());
    }
    #[test]
    fn same_color_trim_stop_and_empty_blocks() {
        let mut d = design();
        d.blocks[0].runs = vec![
            Run {
                pts: vec![P::default()],
                trim_before: false,
            },
            Run {
                pts: vec![P::new(2.0, 0.0)],
                trim_before: true,
            },
        ];
        d.stitch_shapes = vec![vec![vec![(0, 0)], vec![(0, 1)]]];
        let parts = portions(&d).unwrap();
        assert!(selected(&parts, 0).unwrap().outgoing.unwrap().1);
        assert_eq!(parts[1].start, parts[1].end);
        d.blocks[0].runs[1].trim_before = false;
        assert!(
            !selected(&portions(&d).unwrap(), 0)
                .unwrap()
                .outgoing
                .unwrap()
                .1
        );
        let run = d.blocks[0].runs.pop().unwrap();
        d.blocks.push(Block {
            color: [1, 2, 3],
            runs: vec![],
        });
        d.blocks.push(Block {
            color: [1, 2, 3],
            runs: vec![run],
        });
        d.stitch_shapes = vec![vec![vec![(0, 0)]], vec![], vec![vec![(0, 1)]]];
        assert!(
            selected(&portions(&d).unwrap(), 0)
                .unwrap()
                .outgoing
                .unwrap()
                .1
        );
        assert!(!d.blocks[2].runs[0].trim_before); // projection never mutates exported flags
    }
    #[test]
    fn travel_entry_and_invalid_metadata() {
        let mut d = design();
        d.travels.push(Travel {
            how: TravelHow::Routed,
            shape: 1,
            block: 0,
            run: 0,
            from: 1,
            to: 3,
        });
        assert_eq!(
            selected(&portions(&d).unwrap(), 1).unwrap().first.start.x,
            3.0
        );
        d.stitch_shapes[0][0][1].0 = 10;
        assert!(portions(&d).is_none());
        assert!(portions(&Design::default()).unwrap().is_empty());
        d.stitch_shapes.clear();
        assert!(portions(&d).is_none());
    }
    #[test]
    fn ownership_transfers_across_runs() {
        let mut d = design();
        d.blocks[0].runs.push(Run {
            pts: vec![P::new(6.0, 0.0), P::new(7.0, 0.0)],
            trim_before: false,
        });
        d.stitch_shapes[0][0] = vec![(0, 0), (2, 1)];
        d.stitch_shapes[0].push(vec![(0, 1), (1, 0)]);
        let parts = portions(&d).unwrap();
        let b = selected(&parts, 1).unwrap();
        assert_eq!((b.first.run, b.last.run), (0, 1));
        assert_eq!(b.last.end.x, 6.0);
        assert_eq!(b.outgoing, Some((P::new(7.0, 0.0), false)));
    }

    #[test]
    fn single_owner_across_runs_has_no_phantom() {
        let mut d = design();
        d.stitch_shapes[0][0] = vec![(0, 0)];
        d.blocks[0].runs.push(Run {
            pts: vec![],
            trim_before: true,
        });
        d.stitch_shapes[0].push(vec![]);
        let p = selected(&portions(&d).unwrap(), 0).unwrap();
        assert_eq!((p.incoming, p.outgoing), (None, None));
    }
    #[test]
    fn overlay_gate_transform_and_theme_paint() {
        assert!(!crate::view_tools::ViewTools::default().selected_connections);
        let rect = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 300.0));
        let p = selected(&portions(&design()).unwrap(), 1).unwrap();
        let mut view = View::default();
        let old = view.to_screen(rect, p.first.start);
        view.center = P::new(20.0, 10.0);
        view.scale *= 2.0;
        assert_ne!(old, view.to_screen(rect, p.first.start));
        for pal in [crate::theme::LIGHT, crate::theme::DARK] {
            let luminance = |c: egui::Color32| {
                let rgb = c.to_array();
                rgb[..3]
                    .iter()
                    .zip([0.2126, 0.7152, 0.0722])
                    .map(|(&v, w)| {
                        let s = v as f64 / 255.0;
                        w * if s <= 0.04045 {
                            s / 12.92
                        } else {
                            ((s + 0.055) / 1.055).powf(2.4)
                        }
                    })
                    .sum::<f64>()
            };
            for color in [pal.accent, pal.text, pal.secondary] {
                let a = luminance(color);
                let b = luminance(pal.dock);
                assert!((a.max(b) + 0.05) / (a.min(b) + 0.05) >= 3.0);
            }
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(Default::default(), |ui| {
                paint(
                    &ui.ctx().layer_painter(egui::LayerId::background()),
                    rect,
                    view,
                    pal,
                    &p,
                );
            });
            output.textures_delta.clear();
            assert!(!output.shapes.is_empty());
            assert!(output.shapes.iter().all(|s| s.clip_rect == rect));
        }
    }
}

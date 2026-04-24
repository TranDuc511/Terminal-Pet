use ratatui::layout::Rect;

// ─── Utility ──────────────────────────────────────────────────────────────

/// Return a centred `Rect` that is `percent_x`% of `r`'s width and
/// at most `height` rows tall, vertically centred in `r`.
pub(crate) fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect::new(x, y, popup_width.min(r.width), height.min(r.height))
}

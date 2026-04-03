use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    app::{document::WorkspaceDocument, state::AppState},
    domain::minecraft::server::WorkspacePane,
    tui::{components::file_tree, frame::titled_block, theme::Theme},
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, document: &WorkspaceDocument) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let lists = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[0]);

    file_tree::render(
        frame,
        lists[0],
        WorkspacePane::Players.title(),
        &document.server.player_entries,
        document.selection.player_index,
        state.focus == crate::app::state::FocusArea::HomePlayers,
    );
    file_tree::render(
        frame,
        lists[1],
        WorkspacePane::Stats.title(),
        &document.server.stats_entries,
        document.selection.stats_index,
        state.focus == crate::app::state::FocusArea::HomeStats,
    );
    file_tree::render(
        frame,
        lists[2],
        WorkspacePane::Advancements.title(),
        &document.server.advancements_entries,
        document.selection.advancements_index,
        state.focus == crate::app::state::FocusArea::HomeAdvancements,
    );

    let total_entries = document.server.player_entries.len()
        + document.server.stats_entries.len()
        + document.server.advancements_entries.len();

    let info = if total_entries == 0 {
        vec![
            Line::raw(
                "Open a server root with --world to browse playerdata, stats, and advancements.",
            ),
            Line::raw(""),
            Line::raw(
                "You can also open standalone files directly with --player, --nbt, --stats, or --advancements.",
            ),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("World: ", Theme::muted()),
                Span::raw(document.server.level_name.clone()),
            ]),
            Line::from(vec![
                Span::styled("Root:  ", Theme::muted()),
                Span::raw(document.server.root.display().to_string()),
            ]),
            Line::from(vec![
                Span::styled("Mode:  ", Theme::muted()),
                Span::raw(if document.server.online_mode {
                    "online"
                } else {
                    "offline"
                }),
            ]),
            Line::from(vec![
                Span::styled("Players:", Theme::muted()),
                Span::raw(format!(" {}", document.server.player_entries.len())),
                Span::raw("  "),
                Span::styled("Stats:", Theme::muted()),
                Span::raw(format!(" {}", document.server.stats_entries.len())),
                Span::raw("  "),
                Span::styled("Adv:", Theme::muted()),
                Span::raw(format!(" {}", document.server.advancements_entries.len())),
            ]),
            Line::raw(""),
            Line::raw(
                "Use left/right or tab to move between lists. Press o or Enter to open the selected file.",
            ),
            Line::raw(
                "When online mode is enabled, player names are resolved from Mojang automatically.",
            ),
        ]
    };

    frame.render_widget(
        Paragraph::new(info).block(titled_block("Server Overview")),
        chunks[1],
    );
}

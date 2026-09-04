use crate::model::Model::{Point, Rasterizer, MyStyle, SplineType};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

pub struct AppState {
    pub points: Vec<Point>,
    pub point_groups: Vec<Vec<usize>>,
    pub group_types: Vec<SplineType>,
    pub selected_group: Option<usize>,
    pub selected_point: Option<usize>,
    pub rasterizer: Rasterizer,
    pub buffer: Vec<u32>,
    pub cmap: Vec<char>,
    pub buffer_width: usize,
    pub buffer_height: usize,
    pub current_tool: Tool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tool {
    AddPoint,
    Select,
    Delete,
    Group1,
    Group2,
    Group3,
    Group4,
}

impl AppState {
    pub fn new() -> Self {
        let width = 80;
        let height = 40;
        let buffer = vec![0u32; width * height];
        let cmap = vec![' '; width * height];
        let style = MyStyle::default();        
        Self {points: Vec::new(),point_groups: Vec::new(),group_types: Vec::new(),selected_group: None,selected_point: None,rasterizer: Rasterizer::new(style),buffer,buffer_width: width,buffer_height: height,current_tool: Tool::AddPoint,cmap}
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(Point::new(x, y));
        self.update_rasterizer();
    }

    pub fn delete_point(&mut self, index: usize) {
        if index < self.points.len() {
            self.points.remove(index);
            self.update_rasterizer();
        }
    }

    pub fn create_group(&mut self, group_type: SplineType) {
        let group_size = match group_type {
            SplineType::Point => 1,
            SplineType::Line => 2,
            SplineType::QuadraticBezier => 3,
            SplineType::CubicBezier => 4,
        };

        if self.points.len() >= group_size {
            let start_idx = self.points.len() - group_size;
            let indices: Vec<usize> = (start_idx..self.points.len()).collect();
            self.point_groups.push(indices);
            self.group_types.push(group_type);
            self.update_rasterizer();
        }
    }

    pub fn delete_group(&mut self, index: usize) {
        if index < self.point_groups.len() {
            self.point_groups.remove(index);
            self.group_types.remove(index);
            self.update_rasterizer();
        }
    }

    pub fn update_rasterizer(&mut self) {
        self.rasterizer.clear(&mut self.buffer);
        // Draw groups
        for (i, group) in self.point_groups.iter().enumerate() {
            let group_points: Vec<Point> = group.iter().map(|&idx| self.points[idx].clone()).collect();
            self.rasterizer.draw_spline(&mut self.buffer,&mut self.cmap, self.buffer_width, &group_points, self.group_types[i]);
        }

        // Draw non-grouped points
        let mut in_group = vec![false; self.points.len()];
        for group in &self.point_groups {
            for &idx in group {
                if idx < in_group.len() {
                    in_group[idx] = true;
                }
            }
        }

        for (i, point) in self.points.iter().enumerate() {
            if !in_group[i] {
                self.rasterizer.draw_point(&mut self.buffer,&mut self.cmap, self.buffer_width, point);
            }
        }
    }

    pub fn get_point_at(&self, x: f32, y: f32, threshold: f32) -> Option<usize> {
        for (i, point) in self.points.iter().enumerate() {
            let dist = ((point.x - x).powi(2) + (point.y - y).powi(2)).sqrt();
            if dist < threshold {
                return Some(i);
            }
        }
        None
    }
}

pub fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_state = AppState::new();
    let result = run_app_loop(&mut terminal, &mut app_state);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    result
}

fn run_app_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,app_state: &mut AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app_state))?;
        match event::read()? {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('1') => app_state.current_tool = Tool::Group1,
                    KeyCode::Char('2') => app_state.current_tool = Tool::Group2,
                    KeyCode::Char('3') => app_state.current_tool = Tool::Group3,
                    KeyCode::Char('4') => app_state.current_tool = Tool::Group4,
                    KeyCode::Char('a') => app_state.current_tool = Tool::AddPoint,
                    KeyCode::Char('s') => app_state.current_tool = Tool::Select,
                    KeyCode::Char('d') => app_state.current_tool = Tool::Delete,
                    KeyCode::Char('g') => {
                        match app_state.current_tool {
                            Tool::Group1 => app_state.create_group(SplineType::Point),
                            Tool::Group2 => app_state.create_group(SplineType::Line),
                            Tool::Group3 => app_state.create_group(SplineType::QuadraticBezier),
                            Tool::Group4 => app_state.create_group(SplineType::CubicBezier),
                            _ => {}
                        }
                    }
                    KeyCode::Char('r') => {
                        if let Some(idx) = app_state.selected_group {
                            app_state.delete_group(idx);
                            app_state.selected_group = None;
                        }
                    }
                    KeyCode::Char('x') => {
                        if let Some(idx) = app_state.selected_point {
                            app_state.delete_point(idx);
                            app_state.selected_point = None;
                        }
                    }
                    KeyCode::Char('c') => {
                        app_state.points.clear();
                        app_state.point_groups.clear();
                        app_state.group_types.clear();
                        app_state.selected_group = None;
                        app_state.selected_point = None;
                        app_state.update_rasterizer();
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse) => {
                handle_mouse_event(mouse, app_state);
            }
            _ => {
            }
        }
    }
}

fn handle_mouse_event(mouse: MouseEvent, app_state: &mut AppState) {
    match mouse.kind {
        MouseEventKind::Down(button) => {
            let x = mouse.column as f32;
            let y = mouse.row as f32 - 8.5;
            if button == MouseButton::Left {
                if x < 60.0 {
                    match app_state.current_tool {
                        Tool::AddPoint => {
                            app_state.add_point(x, y);
                        }
                        Tool::Select => {
                            if let Some(idx) = app_state.get_point_at(x, y, 4.0) {
                                app_state.selected_point = Some(idx);
                            }
                        }
                        Tool::Delete => {
                            if let Some(idx) = app_state.get_point_at(x, y, 4.0) {
                                app_state.delete_point(idx);
                            }
                        }
                        _ => {}
                    }
                } else {
                }
            }
        }
        _ => {}
    }
}

fn ui(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref()).split(f.area());
    let canvas = Paragraph::new(render_canvas(app_state)).block(Block::default().borders(Borders::ALL).title("Canvas (Left Click to Add Points)")).wrap(Wrap { trim: false });
    f.render_widget(canvas, chunks[0]);

    let sidebar_chunks = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(0),
        ].as_ref()).split(chunks[1]);

    let tool_text = format!("Current Tool: {:?}", app_state.current_tool);
    let tool_widget = Paragraph::new(tool_text).block(Block::default().borders(Borders::ALL).title("Tool")).alignment(Alignment::Center);
    f.render_widget(tool_widget, sidebar_chunks[0]);

    let tools_text = vec![
        Line::from("Tools:"),
        Line::from("[A] Add Point"),
        Line::from("[S] Select Point"),
        Line::from("[D] Delete Point"),
        Line::from("[1] Group 1 (Point)"),
        Line::from("[2] Group 2 (Line)"),
        Line::from("[3] Group 3 (Quad Bezier)"),
        Line::from("[4] Group 4 (Cubic Bezier)"),
    ];

    let tools_widget = Paragraph::new(tools_text).block(Block::default().borders(Borders::ALL).title("Tools")).wrap(Wrap { trim: false });
    f.render_widget(tools_widget, sidebar_chunks[1]);

    let actions_text = vec![
        Line::from("Actions:"),
        Line::from("[G] Create Group"),
        Line::from("[R] Remove Group"),
        Line::from("[X] Delete Selected"),
        Line::from("[C] Clear All"),
        Line::from("[Q] Quit"),
    ];
    let actions_widget = Paragraph::new(actions_text).block(Block::default().borders(Borders::ALL).title("Actions")).wrap(Wrap { trim: false });
    f.render_widget(actions_widget, sidebar_chunks[2]);

    let mut groups_text = vec![Line::from("Groups:")];
    for (i, group) in app_state.point_groups.iter().enumerate() {
        let group_type = match app_state.group_types.get(i) {
            Some(SplineType::Point) => "Point",
            Some(SplineType::Line) => "Line",
            Some(SplineType::QuadraticBezier) => "Quad",
            Some(SplineType::CubicBezier) => "Cubic",
            None => "Unknown",
        };
        let selected = if app_state.selected_group == Some(i) { "*" } else { " " };
        groups_text.push(Line::from(format!("{} [{}] {}: {:?}", selected, i, group_type, group)));
    }
    let groups_widget = Paragraph::new(groups_text).block(Block::default().borders(Borders::ALL).title("Point Groups")).wrap(Wrap { trim: false });
    f.render_widget(groups_widget, sidebar_chunks[3]);

    let mut points_text = vec![Line::from(format!("Points: {}", app_state.points.len()))];
    if let Some(idx) = app_state.selected_point {
        if idx < app_state.points.len() {
            let p = &app_state.points[idx];
            points_text.push(Line::from(format!("Selected: ({:.1}, {:.1})", p.x, p.y)));
        }
    }
    let points_widget = Paragraph::new(points_text).block(Block::default().borders(Borders::ALL).title("Points")).wrap(Wrap { trim: false });
    f.render_widget(points_widget, sidebar_chunks[4]);
}

fn render_canvas(app_state: &AppState) -> Vec<Line> {
    let mut lines = Vec::new();

    for y in 0..app_state.buffer_height {
        let mut spans = Vec::new();
        for x in 0..app_state.buffer_width {
            let idx = y * app_state.buffer_width + x;
            let pixel = app_state.buffer[idx];
            if pixel == 0 {
                spans.push(Span::styled(" ", Style::default()));
            } else {
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                
                let color = Color::Rgb(r, g, b);
                spans.push(Span::styled(format!("{}",app_state.cmap[idx]), Style::default().fg(color)));
            }
        }
        lines.push(Line::from(spans));
    }
    
    lines
}

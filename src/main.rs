extern crate hemoglobin;
extern crate num;
extern crate rustty;

use std::time::Duration;
use std::env;

use rustty::{Terminal, Event, HasSize};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};

fn main() {
    let args: Vec<String> = env::args().collect();
    let rule_arg = &args[1];  // 0th arg is program name
    let rule = hemoglobin::Rule::from(rule_arg.to_owned());

    //Create terminal and canvas
    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);

    let (width, height) = canvas.size();
    let mut w = hemoglobin::World::new(width, height, rule);
    w.gen();

    let mut auto = false;
    let mut delay;

    'rendering: loop {
        if auto {
            delay = 0;
        } else {
            delay = 10;
        }
        while let Some(Event::Key(c)) =
            term.get_event(Some(Duration::from_millis(delay)).unwrap())
                .unwrap()
        {
            match c {
                'q' => break 'rendering,
                'g' => w.gen(),
                'n' => w.step(),
                'a' => auto = true,
                's' => auto = false,
                _ => {}
            }
        }
        if auto {
            w.step();
        }
        w.render(&mut canvas);
        canvas.draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

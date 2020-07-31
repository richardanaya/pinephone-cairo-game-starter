use cairo::Context;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Input {
    pub x: f64,
    pub y: f64,
    pub is_down: bool,
}

pub fn run_game<T>(run: T)
where
    T: 'static + Fn(&Context, &Input, (i32,i32)),
{
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // we embed our image,glade file, and css  in a glib gresource file generated
    //  from app.xml, let's load it in from bytes embedded in our app
    let bytes = glib::Bytes::from_static(include_bytes!("app.gresource"));
    let res = gio::Resource::from_data(&bytes).unwrap();
    gio::resources_register(&res);

    // lets generate all our controls from glade file
    let builder = gtk::Builder::from_resource("/app/app.glade");

    // grab the controls we'll be using
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let event_box: gtk::EventBox = builder.get_object("event_box").unwrap();
    let canvas: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.get_object("canvas").unwrap()));

    let input = Rc::new(RefCell::new(Input {
        x: 0.0,
        y: 0.0,
        is_down: false,
    }));

    let input2 = input.clone();
    let canvas2 = canvas;
    // handle draw and use cairo context
    canvas.borrow_mut().connect_draw(move |_, ctx| {
        run(ctx, &input2.borrow(),(canvas2.get_allocated_width(),canvas2.get_allocated_height()));
        Inhibit(false)
    });

    let input3 = input.clone(); 
    event_box.connect_button_press_event(move |_, e| {
        let mut inp = input3.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.is_down = true;
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    let input4 = input.clone();
    event_box.connect_button_release_event(move |_, e| {
        let mut inp = input4.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.is_down = false;
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    // show the window
    window.show_all();

    // don't maximize in debug (we assume debug is desktop)
    // on phone we should maximize
    #[cfg(not(debug_assertions))]
    window.maximize();

    let canvas2 = canvas.clone();
    let tick = move || {
        canvas2.borrow_mut().queue_draw();
        glib::Continue(true)
    };

    // executes the game every 60 seconds
    gtk::timeout_add(1000 / 60, tick);

    gtk::main();
}

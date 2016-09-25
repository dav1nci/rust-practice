#[macro_use]extern crate conrod;
extern crate piston_window;
extern crate find_folder;

//use conrod::*;
//use piston_window::*;

widget_ids! {
    struct Ids {
        canvas,
        login_field,
        password_field,
        ok_button,
    }
}


fn main() {
    const HEIGHT: u32 = 480;
    const WIDTH: u32 = 720;


    use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};


    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Primitives Demo", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // A unique identifier for each widget.
    //widget_ids!(struct Ids { canvas, circle });
    let mut ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    //Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    let mut count = 0;


    let ref mut login = "".to_string();
    let password = &mut "".to_string();

    while let Some(event) = window.next() {
        // Convert the piston event to conrod event
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let ui = &mut ui.set_widgets(); // UiCell
            set_widg(ui, &mut ids, &mut count, login, password);
        });

        // Draw our Ui!
        //
        // The `draw_if_changed` method only re-draws the GUI if some `Widget`'s `Element`
        // representation has changed. Normally, a `Widget`'s `Element` should only change
        // if a Widget was interacted with in some way, however this is up to the `Widget`
        // designer's discretion.
        //
        // If instead you need to re-draw your conrod GUI every frame, use `Ui::draw`.

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { 
                    img 
                };

                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}


fn set_widg(ui: &mut conrod::UiCell, ids: &mut Ids, count: &mut u32, login: &mut String, password: &mut String){
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
    use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
    use conrod::widget::{Canvas, Line};


    Canvas::new()
        .pad_top(150.0)
        .rgb(1.0, 0.73, 1.0)
        .set(ids.canvas, ui);
    //Line::centred([-40.0, -40.0], [40.0, 40.0]).top_left_of(ids.canvas).set(ids.circle, ui);


    // Draw login filed
    for event in widget::TextBox::new(login)
        //.and_if(true, |text| text.xy([1.0, 1.0]))
        .mid_top_of(ids.canvas)
        .font_size(20)
        .w_h(320.0, 40.0)
        .border(3.0)
        .border_rgb(0.85, 0.43, 0.57)
        .rgb(0.8, 0.75, 0.77)
        .set(ids.login_field, ui) {
            match event {
                widget::text_box::Event::Enter => println!("TextBox : {:?}", login),
                widget::text_box::Event::Update(string) => {
                    println!("login update <{}>", string);
                    *login = string;
                },
        }
    }

    // Draw password field
    for event in widget::TextBox::new(password)
        //.and_if(true, |text| text.xy([1.0, 1.0]))
        .down_from(ids.login_field, 20.0)
        .font_size(20)
        .w_h(320.0, 40.0)
        .border(3.0)
        .border_rgb(0.85, 0.43, 0.57)
        .rgb(0.8, 0.75, 0.77)
        .set(ids.password_field, ui) {
            match event {
                widget::text_box::Event::Enter => println!("TextBox : {:?}", password),
                widget::text_box::Event::Update(string) => {
                    println!("password update <{}>", string);
                    *password = string;
                },
        }
    }

    // Draw `Sign In` button
    for event in widget::Button::new()
        .middle_of(ids.canvas)
        .w_h(80.0, 40.0)
        .label("Sign In")
        .rgb(0.4, 0.4, 0.2)
        .set(ids.ok_button, ui) {
            println!("Button pressed!");
            *count += 1;
        }
}







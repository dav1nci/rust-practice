#[macro_use]extern crate conrod;
extern crate piston_window;
extern crate find_folder;
use std::io;

widget_ids! {
    struct Ids {
        canvas,
        login_field,
        password_field,
        ok_button,
        user_list,
        add_new_user,
        text_not_found,
        left_col,
        right_col,
        change_pass_btn,
        list_reg_users_btn,
        add_new_user_btn,
        change_btn,
        change_status_text,
        user_limit_list,
        exit_btn,
    }
}

enum State {
    login,
    admin_panel,
    user_panel,
}

enum AdminPanelState {
    none,
    change_pass,
    list_users,
    add_new_user,
}

struct Person {
    name: String,
    password: String,
    blocked: bool,
    limit: bool,
}

impl Person {
    fn new() -> Self {
        Person {
            name: "".to_string(),
            password: "".to_string(),
            blocked: false,
            limit: false,
        }
    }

    fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }

    fn is_blocked(mut self, is_blocked: bool) -> Self {
        self.blocked = is_blocked;
        self
    }

    fn is_limit(mut self, is_limit: bool) -> Self {
        self.limit = is_limit;
        self
    }
}

fn get_persons_from_file() -> Result<Vec<Person>, io::Error> {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    use std::str::FromStr;
    let mut f = try!(File::open("resource/userlist.txt"));
    let mut persons = Vec::new();

    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line.unwrap();
        let mut words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 2 {

            // If this is an ADMIN
            let temp: Person = Person::new()
                .name(words[0].to_string())
                .password(words[1].to_string());
            persons.push(temp);
        } else if words.len() == 3 { 

            // This is user without password
            let mut temp: Person = Person::new()
                .name(words[0].to_string())
                .is_blocked(FromStr::from_str(words[1]).unwrap())
                .is_limit(FromStr::from_str(words[2]).unwrap());
            persons.push(temp);
        } else {
            let mut temp: Person = Person::new()
                .name(words[0].to_string())
                .password(words[1].to_string())
                .is_blocked(FromStr::from_str(words[2]).unwrap())
                .is_limit(FromStr::from_str(words[3]).unwrap());
            persons.push(temp);
        }
    }
    for person in &persons {
        println!("name = {}, pass = {}, is_blocked = {}, limit = {}", person.name, person.password, person.blocked, person.limit);
    }
    Ok(persons)
}

fn main() {
    const HEIGHT: u32 = 480;
    const WIDTH: u32 = 720;

    let mut persons = match get_persons_from_file() {
        Ok(persons) => persons,
        Err(e) => panic!("Error when getting persons from file"),
    };


    //match read_from_file(lines) {
    //    Ok(lines) => {
    //        println!("OK!");
    //        for line in lines {
    //            println!("{}", line.unwrap());
    //        }
    //    },
    //    Err(e) => println!("ERR!"),
    //}

    use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("SUPER GUI", [WIDTH, HEIGHT])
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


    // Variables, needed in draw <login_panel>'s function to change state 
    let mut state: State = State::login;
    let mut count = 0;
    let ref mut login = "".to_string();
    let password = &mut "".to_string();
    let is_incorrect: &mut bool = &mut false;
    let is_login_pressed: &mut bool = &mut false;
    let mut err_msg = "".to_string();
    let mut incorrect_attempts_counter: u32 = 2;

    //Variables needed in draw <admin_panel>'s function
    let admin_panel_state: &mut AdminPanelState = &mut AdminPanelState::none;
    let old_password = &mut "".to_string();
    let new_password = &mut "".to_string();
    let change_status = &mut "".to_string();
    let mut new_user_name = "".to_string();

    // Variables needed in draw <user_panel> 
    let mut user_index: usize = 0;

    while let Some(event) = window.next() {
        // Convert the piston event to conrod event
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let ui = &mut ui.set_widgets(); // UiCell
            match state {
                State::login => login_panel(ui, &mut ids, &mut count, login, password, is_incorrect, is_login_pressed, &err_msg),
                State::admin_panel => admin_panel(ui, &mut ids, &mut count, &mut persons, admin_panel_state, old_password, new_password, change_status, &mut new_user_name, &mut state),
                State::user_panel => user_panel(ui, &mut ids, &mut persons, old_password, new_password, change_status, &mut state, user_index),
            }

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

        if *is_login_pressed {
            println!("Login = <{}>, password = <{}>", *login, *password);
            *is_login_pressed = false;
            if *login.as_str() == persons[0].name { // if login ADMIN
                if *password.as_str() == persons[0].password { // If ADMIN's password correct
                    println!("ADMIN ENTERED!");
                    state = State::admin_panel;
                } else {
                    if incorrect_attempts_counter > 0 {
                        *is_incorrect = true;
                        err_msg = format!("Incorrect admin password, <{}> attempts left", incorrect_attempts_counter);
                        incorrect_attempts_counter-=1;
                    } else {
                        err_msg = "3 times incorrect password, gg wp, hacker".to_string();
                        println!("{}", err_msg);
                    }
                }
            } else { // Check if it is user from userlist 
                for i in 0..persons.len()  {
                    if *login.as_str() == persons[i].name { // Login right, check the password
                        if *password.as_str() == persons[i].password { // Password right, check is_block status
                            if !persons[i].blocked { // User don't blocked
                                println!("USER ENTERED!");
                                state = State::user_panel;
                                user_index = i;
                            } else {
                                println!("user <{}> blocked", persons[i].name);
                                err_msg = format!("{} is blocked", persons[i].name);
                            }
                        }
                    }
                }
            }
        }
    }
}


fn admin_panel(ui: &mut conrod::UiCell,
               ids: &mut Ids,
               count: &mut u32,
               persons: &mut Vec<Person>, 
               admin_panel_state: &mut AdminPanelState,
               old_password: &mut String,
               new_password: &mut String,
               change_status: &mut String,
               new_user_name: &mut String,
               state: &mut State) 
{
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
    use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
    use conrod::widget::{Canvas, Line};

    widget::Canvas::new().flow_right(&[
                                     (ids.left_col, widget::Canvas::new().color(color::BLACK).length_weight(1.0)),
                                     (ids.right_col, widget::Canvas::new().color(color::DARK_CHARCOAL).length_weight(2.0)),
                                     ]).set(ids.canvas, ui);


    // Draw `Change pass button` button
    for event in widget::Button::new()
        .mid_top_of(ids.left_col)
            .w_h(150.0, 40.0)
            .label("Change password")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.change_pass_btn, ui) {
                *admin_panel_state = AdminPanelState::change_pass;
            }

    // List registered users btn
    for event in widget::Button::new()
        .down_from(ids.change_pass_btn, 20.0)
            .w_h(150.0, 40.0)
            .label("List all users")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.list_reg_users_btn, ui) {
                *admin_panel_state = AdminPanelState::list_users;
            }

    // Add new user btn
    for event in widget::Button::new()
        .down_from(ids.list_reg_users_btn, 20.0)
            .w_h(150.0, 40.0)
            .label("Add new user")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.add_new_user_btn, ui) {
                *admin_panel_state = AdminPanelState::add_new_user;
            }

    // Add new user btn
    for event in widget::Button::new()
        .mid_bottom_of(ids.left_col)
            .w_h(150.0, 40.0)
            .label("Exit to Login Menu")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.exit_btn, ui) {
                *admin_panel_state = AdminPanelState::none;
                *state = State::login;
            }

    match *admin_panel_state {
        AdminPanelState::change_pass => {

            widget::Text::new("Enter old password: ")
                .color(color::LIGHT_RED)
                .top_left_with_margin_on(ids.right_col, 20.0)
                .align_text_left()
                .line_spacing(10.0)
                .set(ids.text_not_found, ui); // don't care about id in this label

            for event in widget::TextBox::new(old_password)
                .down_from(ids.text_not_found, 20.0)
                    .font_size(20)
                    .w_h(320.0, 40.0)
                    .border(3.0)
                    .border_rgb(0.85, 0.43, 0.57)
                    .rgb(0.8, 0.75, 0.77)
                    .set(ids.login_field, ui) {
                        match event {
                            widget::text_box::Event::Enter => println!("TextBox : {:?}", old_password),
                            widget::text_box::Event::Update(string) => {
                                println!("old_asspword update <{}>", string);
                                *old_password = string;
                            },
                        }
                    }

            widget::Text::new("Enter new password: ")
                .color(color::LIGHT_RED)
                .down_from(ids.login_field, 20.0)
                .align_text_left()
                .line_spacing(10.0)
                .set(ids.add_new_user, ui); // don't care about id in this label

            for event in widget::TextBox::new(new_password)
                .down_from(ids.add_new_user, 20.0)
                    .font_size(20)
                    .w_h(320.0, 40.0)
                    .border(3.0)
                    .border_rgb(0.85, 0.43, 0.57)
                    .rgb(0.8, 0.75, 0.77)
                    .set(ids.password_field, ui) {
                        match event {
                            widget::text_box::Event::Enter => println!("TextBox : {:?}", new_password),
                            widget::text_box::Event::Update(string) => {
                                println!("new_password update <{}>", string);
                                *new_password = string;
                            },
                        }
                    }


            // Add change btn
            for event in widget::Button::new()
                .down_from(ids.password_field, 20.0)
                    .w_h(150.0, 40.0)
                    .label("Change")
                    .rgb(0.4, 0.4, 0.2)
                    .set(ids.change_btn, ui) {
                        if *old_password.as_str() == persons[0].password {
                            persons[0].password = (*new_password).clone().to_string();
                            println!("Password changed, now, admin have password: <{}>", persons[0].password);
                            *change_status = "Pasword changed succesfully".to_string();
                            save_to_file(persons);
                        } else {
                            println!("old password incorrect");
                            *change_status = "Old password incorrect".to_string();
                        }
                    }

            widget::Text::new(&*(*change_status))
                .color(color::LIGHT_RED)
                .down_from(ids.change_btn, 20.0)
                .align_text_left()
                .line_spacing(10.0)
                .set(ids.change_status_text, ui); // don't care about id in this label
        },
        AdminPanelState::list_users => {

            // List for block/unblock access to program
            let (mut items, scrollbar) = widget::List::new(persons.len(), 50.0)
                .scrollbar_on_top()
                .middle_of(ids.right_col)
                .wh_of(ids.right_col)
                .set(ids.user_list, ui);

            while let Some(item) = items.next(ui) {
                let i = item.i;
                let label = format!("{}. {} pass <{}> blocked:<{}> limit:<{}>", i, persons[i].name, persons[i].password, persons[i].blocked, persons[i].limit);
                let toggle = widget::Toggle::new(persons[i].blocked)
                    .label(&label)
                    .label_color(conrod::color::WHITE)
                    .color(conrod::color::LIGHT_BLUE);
                for v in item.set(toggle, ui) {
                    persons[i].blocked = v;
                }
            }

            if let Some(s) = scrollbar { s.set(ui) }

        },
        AdminPanelState::add_new_user => {

            widget::Text::new("Etnter name for new user")
                .color(color::LIGHT_RED)
                .top_left_with_margin_on(ids.right_col, 20.0)
                .align_text_left()
                .line_spacing(10.0)
                .set(ids.text_not_found, ui); // don't care about id in this label

            for event in widget::TextBox::new(new_user_name)
                .down_from(ids.text_not_found, 20.0)
                    .font_size(20)
                    .w_h(320.0, 40.0)
                    .border(3.0)
                    .border_rgb(0.85, 0.43, 0.57)
                    .rgb(0.8, 0.75, 0.77)
                    .set(ids.password_field, ui) {
                        match event {
                            widget::text_box::Event::Enter => println!("TextBox : {:?}", old_password),
                            widget::text_box::Event::Update(string) => {
                                println!("old_asspword update <{}>", string);
                                *new_user_name = string;
                            },
                        }
                    }
            // Add change btn
            for event in widget::Button::new()
                .down_from(ids.password_field, 20.0)
                    .w_h(150.0, 40.0)
                    .label("Add")
                    .rgb(0.4, 0.4, 0.2)
                    .set(ids.change_btn, ui) {

                        let temp: Person = Person::new()
                            .name((*new_user_name).to_string());
                        persons.push(temp);
                        println!("persons now:");
                        for person in &(*persons) {
                            println!("name = {}, pass = {}, is_blocked = {}, limit = {}", person.name, person.password, person.blocked, person.limit);
                        }
                        save_to_file(persons);
                    }
        },
        AdminPanelState::none => {},
    }
}

fn user_panel(ui: &mut conrod::UiCell,
              ids: &mut Ids,
              persons: &mut Vec<Person>,
              old_password: &mut String,
              new_password: &mut String,
              change_status: &mut String,
              state: &mut State, 
              user_index: usize)
{
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
    use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
    use conrod::widget::{Canvas, Line};

    widget::Canvas::new().flow_right(&[
                                     (ids.left_col, widget::Canvas::new().color(color::BLACK).length_weight(1.0)),
                                     (ids.right_col, widget::Canvas::new().color(color::DARK_CHARCOAL).length_weight(2.0)),
                                     ]).set(ids.canvas, ui);

    for event in widget::Button::new()
        .mid_bottom_of(ids.left_col)
            .w_h(150.0, 40.0)
            .label("Exit to Login Menu")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.exit_btn, ui) {
                *state = State::login;
            }


    widget::Text::new("Enter old password: ")
        .color(color::LIGHT_RED)
        .top_left_with_margin_on(ids.right_col, 20.0)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.text_not_found, ui); // don't care about id in this label

    for event in widget::TextBox::new(old_password)
        .down_from(ids.text_not_found, 20.0)
            .font_size(20)
            .w_h(320.0, 40.0)
            .border(3.0)
            .border_rgb(0.85, 0.43, 0.57)
            .rgb(0.8, 0.75, 0.77)
            .set(ids.login_field, ui) {
                match event {
                    widget::text_box::Event::Enter => println!("TextBox : {:?}", old_password),
                    widget::text_box::Event::Update(string) => {
                        println!("old_asspword update <{}>", string);
                        *old_password = string;
                    },
                }
            }

    widget::Text::new("Enter new password: ")
        .color(color::LIGHT_RED)
        .down_from(ids.login_field, 20.0)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.add_new_user, ui); // don't care about id in this label

    for event in widget::TextBox::new(new_password)
        .down_from(ids.add_new_user, 20.0)
            .font_size(20)
            .w_h(320.0, 40.0)
            .border(3.0)
            .border_rgb(0.85, 0.43, 0.57)
            .rgb(0.8, 0.75, 0.77)
            .set(ids.password_field, ui) {
                match event {
                    widget::text_box::Event::Enter => println!("TextBox : {:?}", new_password),
                    widget::text_box::Event::Update(string) => {
                        println!("new_password update <{}>", string);
                        *new_password = string;
                    },
                }
            }


    // Add change btn
    for event in widget::Button::new()
        .down_from(ids.password_field, 20.0)
            .w_h(150.0, 40.0)
            .label("Change")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.change_btn, ui) {
                if *old_password.as_str() == persons[user_index].password {
                    persons[user_index].password = (*new_password).clone().to_string();
                    println!("Password changed, now <{}> have password: <{}>", persons[user_index].name, persons[user_index].password);
                    *change_status = "Pasword changed succesfully".to_string();
                    save_to_file(persons);
                } else {
                    println!("old password incorrect");
                    *change_status = "Old password incorrect".to_string();
                }
            }

    widget::Text::new(&*(*change_status))
        .color(color::LIGHT_RED)
        .down_from(ids.change_btn, 20.0)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.change_status_text, ui); // don't care about id in this label
}

fn login_panel(ui: &mut conrod::UiCell, 
               ids: &mut Ids, 
               count: &mut u32, 
               login: &mut String, 
               password: &mut String,
               is_incorrect: &mut bool, 
               is_login_pressed: &mut bool,
               err_msg: &String) 
{
    use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
    use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
    use conrod::widget::{Canvas, Line};


    Canvas::new()
        .pad_top(150.0)
        .rgb(1.0, 0.73, 1.0)
        .set(ids.canvas, ui);


    for event in widget::TextBox::new(login)
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

    widget::Text::new(err_msg)
        .color(color::LIGHT_RED)
        .padded_w_of(ids.password_field, 20.0)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.text_not_found, ui);

    // Draw `Sign In` button
    for event in widget::Button::new()
        .middle_of(ids.canvas)
            .w_h(80.0, 40.0)
            .label("Sign In")
            .rgb(0.4, 0.4, 0.2)
            .set(ids.ok_button, ui) {
                *is_login_pressed = true;
            }
}

fn save_to_file(persons: &Vec<Person>) -> Result<(), io::Error> {
    use std::fs::File;
    use std::fs::remove_file;
    use std::io::Write;
    use std::clone::Clone;

    try!(remove_file("resource/userlist.txt"));

    let mut f = try!(File::create("resource/userlist.txt"));

    let mut line = String::new();

    for person in persons {
        if person.name == "ADMIN" { // Means admin
            line.clear();
            line.push_str(person.name.as_str());
            line.push(' ');
            line.push_str(person.password.as_str());
            line.push('\n');
        } else if person.password == "" {  // Means new user
            line.clear();
            line.push_str(person.name.as_str());
            line.push(' ');

            match person.blocked {
                true => {
                    line.push_str("true");
                    line.push(' ');
                },
                false => {
                    line.push_str("false");
                    line.push(' ');
                },
            }
            match person.limit {
                true => {
                    line.push_str("true\n");
                },
                false => {
                    line.push_str("false\n");
                },
            }

        } else { // Means user
            line.clear();
            line.push_str(person.name.as_str());
            line.push(' ');
            line.push_str(person.password.as_str());
            line.push(' ');

            match person.blocked {
                true => {
                    line.push_str("true");
                    line.push(' ');
                },
                false => {
                    line.push_str("false");
                    line.push(' ');
                },
            }
            match person.limit {
                true => {
                    line.push_str("true\n");
                },
                false => {
                    line.push_str("false\n");
                },
            }
        }
        try!(f.write_all(line.clone().into_bytes().as_slice()));
    }
    Ok(())
}




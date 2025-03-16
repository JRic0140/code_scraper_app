use gtk::{prelude::*, Orientation};
use gtk::{Application, ApplicationWindow, Paned};


pub struct CodeScraperApp {
    win: Application,
}

pub trait CodeScraperAppTrait {
    fn new() -> CodeScraperApp;
}


/// # Builder for own application
/// this get the path
fn app_builder<T: Fn(Application) -> Application>(path: String, apply: T) -> Application {
    apply(Application::builder().application_id(path).build())
}

impl CodeScraperAppTrait for CodeScraperApp {
    fn new() -> CodeScraperApp {
        CodeScraperApp {
            win: app_builder("org.example.HelloWorld".to_owned(), |app: Application| {
                app.connect_activate(|app| {
                    // We create the main window.
                    let builder = ApplicationWindow::builder()
                    .application(app)
                    .title("Hello, World!")
                    .default_width(800)
                    .default_height(600)
                    .child(&PrincipalPanel::new())
                    .build();

                    builder.present();
                    
                });
                app.run();
                app
            }),
        }
    }
}


struct PrincipalPanel;

impl PrincipalPanel {

    fn new() -> Paned {
        let side =  Paned::new(
            gtk::Orientation::Horizontal
        );
        let my_box = &gtk::Box::new(Orientation::Vertical, 0);
        let scroled_window = &gtk::ScrolledWindow::new();
        scroled_window.set_child(
            Some(
                
                &main_content_box()

            )
        );
        my_box.append(&gtk::Button::with_label("Start"));
        side.set_start_child(
            Some(
                my_box 
            )
        );
        side.start_child().unwrap().set_width_request(150);
            
        side.set_end_child(
            Some(scroled_window)
            );
        side.end_child().unwrap().set_width_request(400);
        

           return side;
    }

}

fn main_content_box() -> gtk::Box {
    let vertical_box = gtk::Box::new( Orientation::Vertical, 0);
    vertical_box.append(&title_text("Title"));
    return vertical_box;
}

fn title_text(arg: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(arg));
    
    return label;
}
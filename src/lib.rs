pub struct CodeScraperApp{
    // url: String,
    // css_selector: String,
    // coupon_codes: Vec<String>,
    Window: Application,
}

use gtk::subclass::widget::CompositeTemplate;
use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;
const APP_ID: &str = "org.gtk_rs.HelloWorld1";

pub trait CodeScraperAppTrait {
    fn new() -> Self;
    fn create_app(&mut self)->Application;
    fn core_ui(app: &Application);
}

impl CodeScraperAppTrait for CodeScraperApp{
    fn new() -> Self {
        let mut app = CodeScraperApp {
            Window: Application::new(Some(APP_ID), Default::default()),
        };
        app.create_app();
        app
    }

    fn create_app(&mut self) -> Application{

        // Create a new application
        let app = Application::builder()
        .application_id(APP_ID)
        .build();

        // Connect to 'activate' signal
        app.connect_activate(|app| Self::core_ui(app));

        // Run the application
        app.run();
        return app;

     }

    fn core_ui(app: &Application) {
        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .build();
    
        // Present window
        window.present();

    }

}
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo6/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListBox>,
    pub tasks: RefCell<Option<gio::ListStore>>,
    pub settings: OnceCell<Settings>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // Create action to remove done tasks and add to action group "win"
        klass.install_action("win.remove-done-tasks", None, |window, _, _| {
            window.remove_done_tasks();
        });
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

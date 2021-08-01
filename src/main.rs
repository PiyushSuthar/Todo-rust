use yew::prelude::*;

enum Msg {
    Update(String),
    AddTodo,
    DeleteTodo(usize),
}

struct Todo {
    completed: bool,
    text: String,
}
struct App {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: String,
    todos: Vec<Todo>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
            todos: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::AddTodo => {
                self.todos.push(Todo {
                    completed: false,
                    text: self.value.to_string(),
                });
                self.value = String::new();
                true
            }
            Msg::DeleteTodo(id) => {
                self.todos.remove(id);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                // <p>{ self.value }</p>
                <div>
                    <h1>{"Rustodo"}</h1>
                        <input
                            type="text"
                            placeholder="Galvanize rust."
                            value=self.value.to_string()
                            oninput=self.link.callback(|e: InputData|{
                                Msg::Update(e.value)
                            })
                            onkeypress=self.link.batch_callback(|e: KeyboardEvent|{
                                if e.key() == "Enter" {
                                    Some(Msg::AddTodo)
                                } else {None}
                            })
                        />
                        <button onclick=self.link.callback(|_| Msg::AddTodo)>{"Add Todo"}</button>
                </div>
                // <h2>{"Todos:"}</h2>
                <div>
                     <table>
                     <thead>
                        <tr>
                            <th width="20"><input type="checkbox" /></th>
                            <th>{"Todo"}</th>
                            <th>{"Option"}</th>
                        </tr>
                     </thead>
                        <tbody>
                            {
                                for self.todos.iter().enumerate().map(|todo| self.view_todo(todo))
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}

impl App {
    fn view_todo(&self, (id, todo): (usize, &Todo)) -> Html {
        html! {
            <tr>
                <td><input type="checkbox" /></td>
                <td>{todo.text.as_str()}</td>
                <td>
                    <button onclick=self.link.callback(move |_| Msg::DeleteTodo(id))>{"Delete"}</button>
                </td>
            </tr>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}

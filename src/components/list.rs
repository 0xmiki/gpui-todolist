use gpui::prelude::*;
use gpui::div;
use gpui::{Context, Window, Entity, Subscription, Hsla};

use super::item::TodoItem;
use super::input::TextInput;

pub struct TodoList{
    items: Vec<Entity<TodoItem>>,
    next_id: usize,
    input: Entity<TextInput>,
    _subscription: Subscription,
    item_subscriptions: Vec<Subscription>,
}

impl TodoList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {  
        // Create the input component as an entity  
        let input = cx.new(|cx| TextInput::new(window, cx));

        let _subscription = cx.subscribe(&input, |this, _input, text, cx| {  
            this.add_item(text.clone(), cx);  
        });  

        Self {  
            items: Vec::new(),  
            next_id: 0,  
            input,
            _subscription,
            item_subscriptions: Vec::new(),
        }  
    }

    fn add_item(&mut self, text: String, cx: &mut Context<Self>) {  
        // Create a new todo item as an entity  
        let item = cx.new(|cx| TodoItem::new(self.next_id, text, cx));
        let _subscription = cx.subscribe(&item, |this, _item, &id, cx| {
            this.remove_item(id, cx);
        });  
        self.item_subscriptions.push(_subscription);
        self.items.push(item);  
        self.next_id += 1;  
        cx.notify(); // Trigger re-render  
    }  

    fn remove_item(&mut self, id: usize, cx: &mut Context<Self>) {    
        // Find and remove the item and its subscription  
        if let Some(index) = self.items.iter().position(|item| item.read(cx).id == id) {  
            self.items.remove(index);  
          let _ = self.item_subscriptions.remove(index);  
            cx.notify();  
        }  
    }  

}

impl Render for TodoList {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .px_4()
                .py_4()
                .h_full()
                .child(self.input.clone())
                .child(
                    div()
                        .mt_4()
                        .flex()
                        .flex_col()
                        .justify_start()
                        .items_start()
                        .gap_2()
                        .children(self.items.iter().cloned())
                )
                
        }
}
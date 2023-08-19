use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::converastion::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // signal that holds the current conversation
    let (converastion, set_converastion) = create_signal(cx, Conversation::new());

    // action that sends a new messege and updates the conversation
    let send = create_action(cx, move |new_messege: &String| {
        let user_messege = Message {
            text: new_messege.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messeges.push(user_messege);
        });

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="ASK-me"/>
        <ChatArea converastion/>
        <TypeArea send/>
    }
}


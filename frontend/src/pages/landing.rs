use dioxus::prelude::*;
// use dioxus_hooks::*;
// use dioxus_free_icons::Icon;
// use dioxus_free_icons::icons::ld_icons::LdList;
// use dioxus_free_icons::icons::ld_icons::LdGrid;
// use dioxus_free_icons::icons::ld_icons::LdPlusCircle;
// use dioxus_free_icons::icons::ld_icons::LdCalendar;
// use dioxus_free_icons::icons::ld_icons::LdUser;
// use dioxus_free_icons::icons::ld_icons::LdClock;
use serde::{ Deserialize, Serialize };

// Define the Event struct
#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct Event {
    id: u32,
    title: String,
    instructor: String,
    date: String,
    time: String,
    category: String,
    description: String,
}

// Define the NewEvent struct ignoring the 'id' field
#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct NewEvent {
    title: String,
    instructor: String,
    date: String,
    time: String,
    category: String,
    description: String,
}

#[component]
pub fn Landing() -> Element {
// pub fn Landing() -> Element {
    // Initialize state
    let events = use_signal(|| vec![
        Event {
            id: 1,
            title: "Beginner Pole Dance".into(),
            instructor: "Jane Doe".into(),
            date: "2024-10-25".into(),
            time: "18:00".into(),
            category: "Pole Dance".into(),
            description: "Learn the basics of pole dance in a fun and supportive environment.".into(),
        },
        Event {
            id: 2,
            title: "Advanced Yoga Flow".into(),
            instructor: "John Smith".into(),
            date: "2024-10-26".into(),
            time: "10:00".into(),
            category: "Yoga".into(),
            description: "Challenge yourself with advanced yoga poses and flows.".into(),
        },
        Event {
            id: 3,
            title: "HIIT Workout".into(),
            instructor: "Emily Brown".into(),
            date: "2024-10-27".into(),
            time: "19:30".into(),
            category: "Fitness".into(),
            description: "High-intensity interval training to boost your metabolism and strength.".into(),
        },
    ]);

    let is_list_view = use_signal(|| true);

    let new_event = use_signal(|| NewEvent {
        title: "".into(),
        instructor: "".into(),
        date: "".into(),
        time: "".into(),
        category: "".into(),
        description: "".into(),
    });

    // Reference to the create event form for scrolling
    let form_ref = use_node_ref();

    // Handler to create a new event
    let handle_create_event = {
        let events = events.clone();
        let new_event = new_event.clone();
        move |e: FormEvent| {
            e.prevent_default();
            let current_events = events.get().clone();
            let new_id = current_events.len() as u32 + 1;
            let event = Event {
                id: new_id,
                title: new_event().title.clone(),
                instructor: new_event().instructor.clone(),
                date: new_event().date.clone(),
                time: new_event().time.clone(),
                category: new_event().category.clone(),
                description: new_event().description.clone(),
            };
            events.with_mut(|ev| ev.push(event));
            new_event.set(NewEvent {
                title: "".into(),
                instructor: "".into(),
                date: "".into(),
                time: "".into(),
                category: "".into(),
                description: "".into(),
            });
            // Scroll to the form
            if let Some(form) = form_ref.cast::<dioxus::prelude::VNode>() {
                // Implement scrolling if necessary
            }
        }
    };

    // Handler to join an event
    // let handle_join_event = move |event_id: u32| {
    //     // Implement your logic here
    //     println!("Joined event with ID: {}", event_id);
    // };

    // cx.render(rsx! {
    //     div { class: "container mx-auto p-4",
    //         h1 { class: "text-3xl font-bold mb-6", "Polecamp Events" }

    //         div { class: "flex justify-between items-center mb-4",
    //             div { class: "flex items-center space-x-2",
    //                 // Toggle Switch for view mode
    //                 input {
    //                     class: "toggle toggle-accent",
    //                     id: "view-mode",
    //                     r#type: "checkbox",
    //                     checked: "{*is_list_view}",
    //                     onchange: move |evt| {
    //                         is_list_view.set(evt.value.parse::<bool>().unwrap());
    //                     }
    //                 }
    //                 label { r#for: "view-mode",
    //                     if *is_list_view {
    //                         rsx!(List { class: "h-5 w-5" })
    //                     } else {
    //                         rsx!(Grid { class: "h-5 w-5" })
    //                     }
    //                 }
    //             }
    //             button {
    //                 class: "btn btn-primary",
    //                 onclick: move |_| {
    //                     // Scroll to the create event form
    //                     if let Some(form) = form_ref.cast::<dioxus::prelude::VNode>() {
    //                         // Implement scrolling if necessary
    //                     }
    //                 },
    //                 rsx!(
    //                     PlusCircle { class: "mr-2 h-4 w-4" },
    //                     "Create Event"
    //                 )
    //             }
    //         }

            // Events List/Grid
            // div {
            //     class: "{if *is_list_view { "flex flex-col space-y-4" } else { "grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3" }}",
            //     events.get().iter().map(|event| rsx!(
            //         div { class: "card bg-base-100 shadow-xl",
            //             div { class: "card-body",
            //                 h2 { class: "card-title", "{event.title}" }
            //                 p { class: "text-sm text-gray-500", "{event.category}" }
            //                 p { "Instructor: {event.instructor}" }
            //                 p { "Date: {event.date}" }
            //                 p { "Time: {event.time}" }
            //                 p { "{event.description}" }
            //                 div { class: "card-actions justify-end",
            //                     button {
            //                         class: "btn btn-primary",
            //                         onclick: move |_| handle_join_event(event.id),
            //                         "Join Event"
            //                     }
            //                 }
            //             }
            //         }
            //     ))
            // }

            // Create Event Form
        //     form {
        //         ref: form_ref,
        //         onsubmit: handle_create_event,
        //         class: "mt-8 space-y-4",
        //         h2 { class: "text-2xl font-bold", "Create New Event" }
        //         div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
        //             div {
        //                 label { class: "label", r#for: "title",
        //                     span { class: "label-text", "Title" }
        //                 }
        //                 input {
        //                     class: "input input-bordered w-full",
        //                     r#type: "text",
        //                     id: "title",
        //                     value: "{new_event.title}",
        //                     oninput: move |e| {
        //                         new_event.set(NewEvent {
        //                             title: e.value.clone(),
        //                             ..new_event.clone()
        //                         })
        //                     },
        //                     required: true,
        //                 }
        //             }
        //             div {
        //                 label { class: "label", r#for: "instructor",
        //                     span { class: "label-text", "Instructor" }
        //                 }
        //                 input {
        //                     class: "input input-bordered w-full",
        //                     r#type: "text",
        //                     id: "instructor",
        //                     value: "{new_event.instructor}",
        //                     oninput: move |e| {
        //                         new_event.set(NewEvent {
        //                             instructor: e.value.clone(),
        //                             ..new_event.clone()
        //                         })
        //                     },
        //                     required: true,
        //                 }
        //             }
        //             div {
        //                 label { class: "label", r#for: "date",
        //                     span { class: "label-text", "Date" }
        //                 }
        //                 input {
        //                     class: "input input-bordered w-full",
        //                     r#type: "date",
        //                     id: "date",
        //                     value: "{new_event.date}",
        //                     oninput: move |e| {
        //                         new_event.set(NewEvent {
        //                             date: e.value.clone(),
        //                             ..new_event.clone()
        //                         })
        //                     },
        //                     required: true,
        //                 }
        //             }
        //             div {
        //                 label { class: "label", r#for: "time",
        //                     span { class: "label-text", "Time" }
        //                 }
        //                 input {
        //                     class: "input input-bordered w-full",
        //                     r#type: "time",
        //                     id: "time",
        //                     value: "{new_event.time}",
        //                     oninput: move |e| {
        //                         new_event.set(NewEvent {
        //                             time: e.value.clone(),
        //                             ..new_event.clone()
        //                         })
        //                     },
        //                     required: true,
        //                 }
        //             }
        //             div {
        //                 label { class: "label", r#for: "category",
        //                     span { class: "label-text", "Category" }
        //                 }
        //                 select {
        //                     class: "select select-bordered w-full",
        //                     id: "category",
        //                     value: "{new_event.category}",
        //                     onchange: move |e| {
        //                         new_event.set(NewEvent {
        //                             category: e.value.clone(),
        //                             ..new_event.clone()
        //                         })
        //                     },
        //                     required: true,
        //                     option { value: "", disabled: true, selected: true, "Select category" }
        //                     option { value: "Pole Dance", "Pole Dance" }
        //                     option { value: "Yoga", "Yoga" }
        //                     option { value: "Fitness", "Fitness" }
        //                     option { value: "Dance", "Dance" }
        //                 }
        //             }
        //         }
        //         div {
        //             label { class: "label", r#for: "description",
        //                 span { class: "label-text", "Description" }
        //             }
        //             textarea {
        //                 class: "textarea textarea-bordered w-full",
        //                 id: "description",
        //                 value: "{new_event.description}",
        //                 oninput: move |e| {
        //                     new_event.set(NewEvent {
        //                         description: e.value.clone(),
        //                         ..new_event.clone()
        //                     })
        //                 },
        //                 required: true,
        //             }
        //         }
        //         button {
        //             class: "btn btn-primary",
        //             r#type: "submit",
        //             "Create Event"
        //         }
        //     }
        // }
    // })
    rsx!(
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6", "Polecamp Events" }
        }
    )
}
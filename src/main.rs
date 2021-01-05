use {
    chrono::{NaiveDate, NaiveDateTime},
    itertools::Itertools,
    outbox::{ObjectUnion, OrderedItem},
};

#[macro_use]
extern crate serde_derive;

pub mod actor;
pub mod outbox;

use {actor::Actor, heck::SnakeCase, outbox::Outbox};

fn main() {
    let actor: Actor = serde_json::from_str(include_str!("../archive/actor.json")).unwrap();
    let outbox: Outbox = serde_json::from_str(include_str!("../archive/outbox.json")).unwrap();

    print!(
        r#"<!DOCTYPE html>
           <html>
             <head>
               <meta charset="utf-8">
               <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/light.css">
             </head>
             <body>
               <header>{}</header>
               <main>{}</main>
             </body>
           </html>"#,
        actor.to_html(),
        outbox.to_html()
    );
}

impl Outbox {
    fn to_html(&self) -> String {
        let toots: Vec<(NaiveDate, Vec<String>)> = self
            .ordered_items
            .iter()
            .map(|item| item.to_html())
            .into_group_map()
            .into_iter()
            .collect();

        let mut toots = toots;
        toots.sort_by(|(left_date, _left_htmls), (right_date, _right_htmls)| {
            left_date.partial_cmp(right_date).unwrap()
        });

        toots
            .iter()
            .map(|(datetime, htmls)| {
                format!(
                    "<section><h2>{}</h2>{}</section>",
                    datetime,
                    htmls.join("<hr>")
                )
            })
            .join("")
    }
}

impl OrderedItem {
    fn to_html(&self) -> (NaiveDate, String) {
        let datetime =
            NaiveDateTime::parse_from_str(&self.published, "%Y-%m-%dT%H:%M:%SZ").unwrap();

        (
            datetime.date(),
            format!(
                "<section id={}>{}<br><small>{}</small></section>",
                self.id.to_snake_case().trim_end_matches("_activity"),
                self.object.to_html(),
                datetime
            ),
        )
    }
}

impl ObjectUnion {
    fn to_html(&self) -> String {
        use ObjectUnion::*;
        match self {
            ObjectClass(object_class) => {
                format!(
                    "{}<br><small>{}</small>",
                    (match object_class.summary.clone() {
                        None => object_class.content.clone(),
                        Some(summary) => format!(
                            "<dl><dt>{}</dt></dd>{}</dd></dl>",
                            summary, object_class.content
                        ),
                    }),
                    (object_class
                        .in_reply_to
                        .as_ref()
                        .map(|to| format!(r##"<a href="#{}">&gt;</a>"##, to.to_snake_case()))
                        .unwrap_or("".to_string()))
                )
            }
            String(string) => string.to_string(),
        }
    }
}

impl Actor {
    fn to_html(&self) -> String {
        format!(
            "<h1>{}<code>@{}</code></h1> \
             <blockquote>{}</blockquote>",
            self.name, self.preferred_username, self.summary
        )
    }
}

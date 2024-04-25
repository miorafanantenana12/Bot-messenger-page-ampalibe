use core::panic;

use rocket::serde::json::Value;
use rocket::serde::Serialize;

use crate::{core::response::Res as res, Action};

use super::{
    button::Button,
    data::{Data, Page, MAX_PAGE},
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
    recipient::Recipient,
    text::TextModel,
    ResponseModel,
};

#[derive(Debug, Clone, Serialize)]
pub struct GenericElement {
    title: String,
    image_url: String,
    subtitle: String,
    buttons: Vec<Value>,
}

impl GenericElement {
    pub fn new(title: &str, image_url: &str, subtitle: &str, buttons: Vec<Button>) -> Self {
        if buttons.len() > 3 {
            panic!("Buttons must be three maximum")
        }
        let buttons: Vec<_> = buttons.iter().map(|btn| btn.to_value()).collect();
        Self {
            title: title.into(),
            image_url: image_url.into(),
            subtitle: subtitle.into(),
            buttons,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct GenericPayload {
    pub template_type: String,
    pub elements: Vec<GenericElement>,
}

#[derive(Debug, Clone, Serialize)]
struct Attachment {
    #[serde(rename = "type")]
    pub r#type: String,
    pub payload: GenericPayload,
}

#[derive(Debug, Clone, Serialize)]
struct GenericMessage {
    pub attachment: Attachment,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenericModel<'g> {
    recipient: Recipient<'g>,
    messaging_type: &'g str,
    message: GenericMessage,
}

impl<'g> GenericModel<'g> {
    pub fn new(sender: &'g str, mut elements: Vec<GenericElement>, page: Option<Page>) -> Self {
        if let Some(p) = page {
            elements = elements.into_iter().skip(p.0).take(p.1 - p.0).collect();
        } else if elements.len() >= MAX_PAGE {
            elements.truncate(MAX_PAGE);
        }
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE",
            message: GenericMessage {
                attachment: Attachment {
                    r#type: "template".to_owned(),
                    payload: GenericPayload {
                        template_type: "generic".to_owned(),
                        elements,
                    },
                },
            },
        }
    }
}

impl<'g> GenericModel<'g> {
    fn get_sender(&self) -> &'g str {
        self.recipient.id
    }

    fn is_element_empty(&self) -> bool {
        self.message.attachment.payload.elements.is_empty()
    }

    pub async fn send_next<A: Action>(&self, action: A, mut data: Data) {
        if !self.is_element_empty() {
            data.next_page();
            let quick_reply = QuickReply::new("Next", "", Payload::new(action, Some(data)));

            res.send(QuickReplyModel::new(
                self.get_sender(),
                "Navigation",
                vec![quick_reply],
            ))
            .await;
        } else {
            res.send(TextModel::new(self.get_sender(), "No more elements"))
                .await;
        };
    }
}

impl ResponseModel for GenericModel<'_> {
    const END_POINT: &'static str = "messages";
}

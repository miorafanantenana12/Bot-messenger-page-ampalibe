use russenger::{
    create_action,
    generic::{GenericButton, GenericElement, GenericModel},
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
    russenger_app,
    text::TextModel,
    Data, Req, Res,
};

create_action!(Main, |res: Res, req: Req| async move {
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!")).await;

    let replies = vec![
        QuickReply::new("Option1", "", Payload::new(Option1, Some(Data::new("payload_for_option_1", None)))),
        QuickReply::new("Option2", "", Payload::new(Option1, Some(Data::new("payload_for_option_2", None)))),
    ];

    res.send(QuickReplyModel::new(&req.user, "Choose an option:", replies)).await;

});

create_action!(Option1, |res: Res, req: Req| async move {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 1 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;
});

create_action!(Option2, |res: Res, req: Req| async move {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 2 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;

    let generic_elements = vec![GenericElement::new(
        "Option 2",
        "https://example.com/option2.jpg",
        "Option 2 description",
        vec![GenericButton::new(
            "Choose Option 2",
            Payload::new(Main, None),
        )],
    )];

    res.send(GenericModel::new(&req.user, generic_elements))
        .await;
});

russenger_app!(Main, Option1, Option2);

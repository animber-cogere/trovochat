use super::*;

#[test]
fn parse_bad_auth() {
    let input = ":tmi.trovo.tv NOTICE * :Improperly formatted auth";
    let mut stream = crate::teststream::TestStream::new();
    stream.write_message(input);

    let (read, writer) = crate::sync_adapters(stream.clone(), stream);
    let mut client = crate::Client::new(read, writer);

    let err = client.read_message().unwrap_err();
    if let crate::trovo::Error::InvalidRegistration = err {
        return;
    }
    panic!("unexpected error: {}", err)
}

#[test]
fn parse_commands() {
    use crate::trovo::Message as Command;

    let input = ":museun!museun@museun.tmi.trovo.tv JOIN #museun";
    let expected = Command::Join(Join {
        user: "museun".to_string(),
        channel: "#museun".into_channel().unwrap(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":jtv MODE #museun +o shaken_bot";
    let expected = Command::Mode(Mode {
        channel: "#museun".into_channel().unwrap(),
        status: ModeStatus::Gained,
        user: "shaken_bot".into(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":jtv MODE #museun -o shaken_bot";
    let expected = Command::Mode(Mode {
        channel: "#museun".into_channel().unwrap(),
        status: ModeStatus::Lost,
        user: "shaken_bot".into(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input =
        ":museun!museun@museun.tmi.trovo.tv 353 museun = #museun :museun shaken_bot2 shaken_bot3";
    let expected = Command::NamesStart(NamesStart {
        channel: "#museun".into_channel().unwrap(),
        user: "museun".into(),
        users: ["museun", "shaken_bot2", "shaken_bot3"]
            .iter()
            .cloned()
            .map(str::to_string)
            .collect(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":museun!museun@museun.tmi.trovo.tv 353 museun = #museun :shaken_bot4 shaken_bot5";
    let expected = Command::NamesStart(NamesStart {
        channel: "#museun".into_channel().unwrap(),
        user: "museun".into(),
        users: ["shaken_bot4", "shaken_bot5"]
            .iter()
            .cloned()
            .map(str::to_string)
            .collect(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":museun!museun@museun.tmi.trovo.tv 366 museun #museun :End of /NAMES list";
    let expected = Command::NamesEnd(NamesEnd {
        channel: "#museun".into_channel().unwrap(),
        user: "museun".into(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":museun!museun@museun.tmi.trovo.tv PART #museun";
    let expected = Command::Part(Part {
        user: "museun".into(),
        channel: "#museun".into_channel().unwrap(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv CLEARCHAT #museun :shaken_bot";
    let expected = Command::ClearChat(ClearChat {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        user: Some("shaken_bot".into()),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv CLEARCHAT #museun";
    let expected = Command::ClearChat(ClearChat {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        user: None,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv CLEARMSG #museun :HeyGuys";
    let expected = Command::ClearMsg(ClearMsg {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: Some("HeyGuys".into()),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv CLEARMSG #museun";
    let expected = Command::ClearMsg(ClearMsg {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: None,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv HOSTTARGET #shaken_bot #museun 1024";
    let expected = Command::HostTargetStart(HostTargetStart {
        source: "#shaken_bot".into(),
        target: "#museun".into(),
        viewers: Some(1024),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv HOSTTARGET #shaken_bot #museun";
    let expected = Command::HostTargetStart(HostTargetStart {
        source: "#shaken_bot".into(),
        target: "#museun".into(),
        viewers: None,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv HOSTTARGET #shaken_bot :- 1024";
    let expected = Command::HostTargetEnd(HostTargetEnd {
        source: "#shaken_bot".into(),
        viewers: Some(1024),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv HOSTTARGET #shaken_bot :-";
    let expected = Command::HostTargetEnd(HostTargetEnd {
        source: "#shaken_bot".into(),
        viewers: None,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv NOTICE #museun :This room is no longer in slow mode.";
    let expected = Command::Notice(Notice {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: "This room is no longer in slow mode.".into(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv RECONNECT";
    let expected = Command::Reconnect(Reconnect);
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv ROOMSTATE #museun";
    let expected = Command::RoomState(RoomState {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv USERNOTICE #museun :This room is no longer in slow mode.";
    let expected = Command::UserNotice(UserNotice {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: Some("This room is no longer in slow mode.".into()),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":tmi.trovo.tv USERSTATE #museun";
    let expected = Command::UserState(UserState {
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = "GLOBALUSERSTATE";
    let expected = Command::GlobalUserState(GlobalUserState {
        tags: Tags::default(),
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":museun!museun@museun.tmi.trovo.tv PRIVMSG #museun :VoHiYo";
    let expected = Command::PrivMsg(PrivMsg {
        user: "museun".into(),
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: "VoHiYo".into(),
        action: false,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);

    let input = ":museun!museun@museun.tmi.trovo.tv PRIVMSG #museun :ACTION VoHiYo";
    let expected = Command::PrivMsg(PrivMsg {
        user: "museun".into(),
        tags: Tags::default(),
        channel: "#museun".into_channel().unwrap(),
        message: "VoHiYo".into(),
        action: true,
    });
    assert_eq!(parse(&Message::parse(input).unwrap()).unwrap(), expected);
}

#[test]
fn parse_badge_info() {
    let input = "@badge-info=subscriber/13;badges=subscriber/12;color=#0000FF;display-name=emilgardis;emote-sets=0;mod=0;subscriber=1;user-type=";
    let tags = crate::Tags::parse(&input);
    assert_eq!(
        badges(tags.get("badge-info").unwrap_or_default()),
        vec![Badge {
            kind: crate::BadgeKind::Subscriber,
            data: "13".into(),
        }],
    );

    assert_eq!(
        badges(tags.get("badges").unwrap_or_default()),
        vec![Badge {
            kind: crate::BadgeKind::Subscriber,
            data: "12".into(),
        }],
    );
}

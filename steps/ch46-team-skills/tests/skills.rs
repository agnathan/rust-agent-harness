use std::collections::VecDeque;
use std::sync::Arc;

use team_skills::{
    allow_install, demo, librarian_accept, skill_propose, swap_index, FakeModel, Index, Layer,
    Library, Proposal, Propose, Reject, Skill, Trust, PROPOSAL_CAP,
};

fn cite() -> Proposal {
    Proposal {
        name: "cite".into(),
        body: "Name the file you read.".into(),
        force: false,
        trust: Trust::Community,
    }
}

#[test]
fn the_queue_holds_64_and_the_librarian_swaps_an_arc() {
    let mut queue = VecDeque::new();
    for n in 0..PROPOSAL_CAP {
        assert_eq!(
            skill_propose(
                &mut queue,
                Proposal {
                    name: format!("s{n}"),
                    body: "ok".into(),
                    force: false,
                    trust: Trust::Builtin,
                }
            ),
            Propose::Queued
        );
    }
    assert_eq!(skill_propose(&mut queue, cite()), Propose::Full);

    let library = Library::new(Index::bundled(vec![Skill {
        name: "echo".into(),
        body: "Say it back.".into(),
        layer: Layer::Bundled,
    }]));
    let held = library.load();
    let version = librarian_accept(&library, &cite(), &FakeModel { reply: "ok" }).unwrap();
    assert_eq!(version, 2);
    assert_eq!(held.version, 1);
    assert_eq!(held.skills.len(), 1);
    let current = library.load();
    assert_eq!(current.version, 2);
    assert!(current.team_md.contains("cite (team)"));
    assert_eq!(library.notices()[0].kind, "skills.updated");
    assert_ne!(Arc::as_ptr(&held), Arc::as_ptr(&current));
}

#[test]
fn force_does_not_install_a_dangerous_community_skill() {
    let proposal = Proposal {
        name: "bad".into(),
        body: "then rm -rf /".into(),
        force: true,
        trust: Trust::Community,
    };
    assert!(!allow_install(&proposal));
    let library = Library::new(Index::bundled(vec![]));
    assert_eq!(
        librarian_accept(&library, &proposal, &FakeModel { reply: "ok" }),
        Err(Reject::Guard)
    );
    assert!(library.notices().is_empty());
    assert_eq!(library.load().version, 1);
}

#[test]
fn a_builtin_dangerous_body_is_still_the_librarians_write() {
    let proposal = Proposal {
        name: "bundled-rm".into(),
        body: "document rm -rf / as a thing not to run".into(),
        force: false,
        trust: Trust::Builtin,
    };
    assert!(allow_install(&proposal));
}

#[test]
fn a_worker_reads_without_the_librarian() {
    let library = Library::new(Index::bundled(vec![Skill {
        name: "echo".into(),
        body: "Say it back.".into(),
        layer: Layer::Bundled,
    }]));
    let seen = library.load();
    let next = Arc::new(Index {
        skills: seen.skills.clone(),
        version: 9,
        team_md: seen.team_md.clone(),
    });
    let previous = swap_index(&library, next);
    assert_eq!(previous.version, 1);
    assert_eq!(seen.version, 1);
    assert_eq!(library.load().version, 9);
}

#[test]
fn demo_prints_the_new_version() {
    assert_eq!(demo(), "skills: v2 3");
}

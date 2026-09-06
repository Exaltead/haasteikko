INSERT INTO event (id, user_id, item_id, completed_at)
SELECT lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||substr(hex(randomblob(2)),2)||'-'||
       substr('89ab',abs(random())%4+1,1)||substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),
       user_id, id, completed_at
FROM library;

INSERT INTO event_challenge (event_id, challenge_id)
SELECT e.id, aic.challenge_id
FROM activated_item_challenge aic
JOIN event e ON e.item_id = aic.item_id;

CREATE TABLE answer_new (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL REFERENCES question(id) ON DELETE CASCADE,
    challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    answer TEXT NOT NULL,
    answered INTEGER NOT NULL DEFAULT 0,
    event_id TEXT REFERENCES event(id) ON DELETE CASCADE
);

INSERT INTO answer_new (id, question_id, challenge_id, user_id, kind, answer, answered, event_id)
SELECT a.id, a.question_id, a.challenge_id, a.user_id, a.kind, a.answer, a.answered, e.id
FROM answer a JOIN event e ON e.item_id = a.item_id;

DROP TABLE answer;
ALTER TABLE answer_new RENAME TO answer;

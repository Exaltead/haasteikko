INSERT INTO event (id, user_id, item_id, completed_at)
SELECT lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||substr(hex(randomblob(2)),2)||'-'||
       substr('89ab',abs(random())%4+1,1)||substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),
       user_id, id, completed_at
FROM library;

INSERT INTO event_challenge (event_id, challenge_id)
SELECT e.id, aic.challenge_id
FROM activated_item_challenge aic
JOIN event e ON e.item_id = aic.item_id;

-- Answers stay keyed by item: a single answer set per (item, challenge), shared by all events.
DELETE FROM answer WHERE item_id IS NULL;

DELETE FROM answer
WHERE id NOT IN (
    SELECT id FROM (
        SELECT id, ROW_NUMBER() OVER (
            PARTITION BY user_id, item_id, question_id
            ORDER BY answered DESC, id
        ) AS rn
        FROM answer
    )
    WHERE rn = 1
);

CREATE UNIQUE INDEX IF NOT EXISTS answer_item_question ON answer (user_id, item_id, question_id);

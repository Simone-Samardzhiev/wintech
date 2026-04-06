-- The password is the same for all users: (Password_123)
INSERT INTO users (id, name, email, password)
VALUES ('550e8400-e29b-41d4-a716-446655440000', 'Alice Freeman', 'alice@example.com',
        '$argon2id$v=19$m=16,t=2,p=1$Vk9aQkI1Wlh1RHpMaEdVWQ$j6k+ju/wmnwuTp/vVo4xwg'),
       ('550e8400-e29b-41d4-a716-446655440001', 'Bob Smith', 'bob@example.com',
        '$argon2id$v=19$m=16,t=2,p=1$Vk9aQkI1Wlh1RHpMaEdVWQ$j6k+ju/wmnwuTp/vVo4xwg'),
       ('550e8400-e29b-41d4-a716-446655440002', 'Charlie Root', 'charlie@example.com',
        '$argon2id$v=19$m=16,t=2,p=1$Vk9aQkI1Wlh1RHpMaEdVWQ$j6k+ju/wmnwuTp/vVo4xwg'),
       ('550e8400-e29b-41d4-a716-446655440003', 'Diana Prince', 'diana@example.com',
        '$argon2id$v=19$m=16,t=2,p=1$Vk9aQkI1Wlh1RHpMaEdVWQ$j6k+ju/wmnwuTp/vVo4xwg'),
       ('550e8400-e29b-41d4-a716-446655440004', 'Edward Norton', 'edward@example.com',
        '$argon2id$v=19$m=16,t=2,p=1$Vk9aQkI1Wlh1RHpMaEdVWQ$j6k+ju/wmnwuTp/vVo4xwg');
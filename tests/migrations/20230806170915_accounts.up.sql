-- Add up migration script here

INSERT INTO accounts.users(id, last_name, first_name, gender, date_of_birth, phc_string, is_farmer, is_staff,
                           is_superuser, last_login, date_joined, account_locked, account_locked_reason, account_locked_until)

    VALUES('6c84590e-455c-4c3e-ae83-e3fe5151d36b', 'Shimakeleni', 'Ester', 'F', '2015-12-12',
           '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
            false, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('def69678-767b-4a02-b5bc-28d31152c851', 'Hamwaalwa', 'Benson', 'M', '2018-01-12',
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          true, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('a7912710-2141-41ef-a037-2d864a8796aa', 'Hamwaalwa', 'Grace', 'F', null,
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          false, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('32844324-941e-47c2-849d-b9b179201e98', 'Hamwaalwa', 'Faith', 'F', '2013-12-17', 
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          false, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', 'Shuumbwa', 'Aune', 'F', '1999-07-20',
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          true, true, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('ac14a0d5-3794-4a6b-a31b-af05eb49e144', 'Hango ', 'Laina', 'F', null, 
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          false, true, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),  

         ('3136d378-5a8f-43b4-923f-c7d2cc8f24fd', 'Hango', 'Johanna', 'F', null, 
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          false, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('1f52b2cf-4d96-4065-bb5b-6be1b8e2dca0', 'Shikongo', 'Maria', 'F', null,
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          true, true, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

         ('5bc31df5-81c3-45a4-94b9-89f40409ae29', 'Sheuya', 'Petrus', 'M', '1999-01-25',
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          true, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null), 

         ('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', 'Elifas', 'Pena', 'F', '1999-09-13',
          '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
          true, false, false, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null),

        -- Superuser
        ('272a485f-a46a-4938-8f1a-bbbbbbbbbbb1', null, 'Oshihenye', 'F', '1999-09-13',
        '$argon2id$v=19$m=4096,t=3,p=1$CYGvaUHzYEEw2Dd8rg7rBg$vWdyf3pKW3fJs5FBlApskL3O8W7K7SrmC7P5MkvQ180',
        false, true, true, '2022-09-05 18:02:25.533896846 +00:00:00', '2022-09-05 12:02:25.533896846 +00:00:00', false, null, null);



INSERT INTO accounts.emails(user_id, email, verified, token, token_generated_at)
    VALUES('6c84590e-455c-4c3e-ae83-e3fe5151d36b', 'shimakeleni.ester@gmail.com', true, null, null),
          ('def69678-767b-4a02-b5bc-28d31152c851', 'hamwaalwa.benson@gmail.com', true, null, null),
          ('a7912710-2141-41ef-a037-2d864a8796aa', 'hamwaalwa.grace@gmail.com', true, null, null),
          ('32844324-941e-47c2-849d-b9b179201e98', 'hamwalwa.faith@gmail.com', true, null, null),
          ('5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', 'ammy.aune@gmail.com', true, null, null),
          ('ac14a0d5-3794-4a6b-a31b-af05eb49e144', 'hango.laina@gmail.com', true, null, null),  
          ('3136d378-5a8f-43b4-923f-c7d2cc8f24fd', 'hango.johanna@gmail.com', false, null, null),
          ('1f52b2cf-4d96-4065-bb5b-6be1b8e2dca0', 'mcy.mirjan@gmail.com', true, null, null),
          ('5bc31df5-81c3-45a4-94b9-89f40409ae29', 'peter.petrus@gmail.com', true, null, null), 
          ('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', 'yabeko.pennie@gmail.com', true, null, null),

            -- Superuser
          ('272a485f-a46a-4938-8f1a-bbbbbbbbbbb1', 'oshihenye@gmail.com', true, null, null);



-- accounts.phone_numbers

INSERT INTO accounts.phones(user_id, phone, verified)
    VALUES('5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', '+264812573850', false),
          ('5bc31df5-81c3-45a4-94b9-89f40409ae29', '+264815608565', false),
          ('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', '+264817204595', false);



-- accounts.follows


-- INSERT INTO accounts.follows(user_id, follows_id)
--     VALUES('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', '5bc31df5-81c3-45a4-94b9-89f40409ae29'),
--           ('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b'),
--           ('5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', '292a485f-a56a-4938-8f1a-bbbbbbbbbbb1'),
--           ('5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b'),
--           ('6c84590e-455c-4c3e-ae83-e3fe5151d36b', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b'),
--           ('a7912710-2141-41ef-a037-2d864a8796aa', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b'),
--           ('32844324-941e-47c2-849d-b9b179201e98', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b'),
--           ('def69678-767b-4a02-b5bc-28d31152c851', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b');
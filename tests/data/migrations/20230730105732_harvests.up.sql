-- Add up migration script here

-- services.harvest
INSERT INTO services.harvests(id, cultivar_id, location_id, price, type, description, available_at, finished,
                              images, updated_at, finished_at, created_at)   
     VALUES('fb98aa81-74ab-4708-94f1-4511bb0fafe5', 'd80ead28-a531-459c-911b-273a97aab929',  '5b793e9f-94f1-4f8c-96d5-8aef167c461c', '{"amount": 60, "unit": {"Kg": 10}}',
      'Avocadoes', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('143127cc-b449-41b8-a12e-82512ad7fef4', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  '5b793e9f-94f1-4f8c-96d5-8aef167c461c', '{"amount": 250, "unit": "Crate"}',
      'Grapes', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),
-- Benson Agri farm cultivars
     ('56aa4b16-21cf-4186-83c9-1b4e6f271a7e', '487fc524-47a2-426c-a289-1234041e590b',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 250, "unit": "Crate"}',
       'Juliet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('107765eb-0086-45ed-9567-2dc723dbebc5', '9a48090c-caae-4c44-aeee-4dd507b6ccbe', '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 5, "unit": {"Kg": 1}}',
       'Crimson sweet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('4fa1e4d0-46a9-48e9-b358-c567366a9114', '63b55154-09eb-4911-b32f-632815cfb3e6',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 70, "unit": {"Kg": 10}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('ae9448c4-bc3b-4519-9ea1-e7293a86b7c6', '6ecdb665-4a50-478e-a67c-c5ee4c22b872', '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 200, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('1c2116f9-fc32-4bbe-81f3-7d345ffa583c', '3251249a-972b-4ae6-aeb3-12721d1bff35',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 20, "unit": {"Kg": 5}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('eff2a2ba-88dc-422f-be39-c33fc653b85f', 'fa5c962a-609f-4f58-b573-25bf29e77788',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('3579ea30-5aff-4954-b5ca-e0d4b542d751', '219f353d-4ab4-44d5-8504-f85b906d0cbd',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('de5f58c8-e428-433e-a4f1-f0a2443b823d', '52304ad3-8ba6-4654-87ec-e53635db4d7c',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 250, "unit": "Crate"}',
      'Green pepper', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('82674ce6-2dae-4069-9f09-4c6a98e1aaa5', 'd80ead28-a531-459c-911b-273a97aab929',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 60, "unit": {"Kg": 10}}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('e5534138-4376-40f9-86b0-d960b5f74134', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  '65e67229-989c-46fa-ab54-0d26ff2d8a18', '{"amount": 250, "unit": "Crate"}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),


-- Ammy Green Vegs cultivars
      ('705d6a6d-c316-47bc-86bd-c3058482087a', '487fc524-47a2-426c-a289-1234041e590b',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 250, "unit": "Crate"}',
       'Juliet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('90ae13f4-8172-4a08-955e-e22b0edca02c', '9a48090c-caae-4c44-aeee-4dd507b6ccbe', 'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 5, "unit": {"Kg": 1}}',
       'Crimson sweet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('8ff91b83-9521-410a-b8ec-ab5868844477', '63b55154-09eb-4911-b32f-632815cfb3e6',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 70, "unit": {"Kg": 10}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('398be3a2-45ce-4c52-92c0-0b075cf8067c', '6ecdb665-4a50-478e-a67c-c5ee4c22b872', 'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 200, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('dd5bb765-8da4-4983-8887-4c884f9be972', '3251249a-972b-4ae6-aeb3-12721d1bff35',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 20, "unit": {"Kg": 5}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('64962c5b-f974-420e-b18a-fcd01ec6a1be', 'fa5c962a-609f-4f58-b573-25bf29e77788',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('9cbb1677-1c50-4899-9287-532eb305cc01', '219f353d-4ab4-44d5-8504-f85b906d0cbd',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('bd318091-eab9-4aca-9365-f209785000dd', '52304ad3-8ba6-4654-87ec-e53635db4d7c',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 250, "unit": "Crate"}',
      'Green pepper', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('1f58d88a-4d20-440a-8beb-7b370fc6fa7b', 'd80ead28-a531-459c-911b-273a97aab929',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 60, "unit": {"Kg": 10}}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('5f9759b4-0cec-4b27-877e-7f9a64f34b50', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  'b9ad294c-37b5-457f-9ddb-6388cb156f74', '{"amount": 250, "unit": "Crate"}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),


-- M-cy Vegatable cultivars

      ('eb5267c1-816c-4569-bd32-31a137abc923', '487fc524-47a2-426c-a289-1234041e590b',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 250, "unit": "Crate"}',
       'Juliet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('a0e21816-d99c-45a6-902e-5f412b7362c8', '9a48090c-caae-4c44-aeee-4dd507b6ccbe', '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 5, "unit": {"Kg": 1}}',
       'Crimson sweet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('8486d386-71ed-40f3-99f7-dc26dfc04b99', '63b55154-09eb-4911-b32f-632815cfb3e6',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 70, "unit": {"Kg": 10}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('c3f3a6a9-1c6b-4eb9-b7e5-24acf856636b', '6ecdb665-4a50-478e-a67c-c5ee4c22b872', '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 200, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('55fb72db-c720-4e25-add3-f0b6626168f2', '3251249a-972b-4ae6-aeb3-12721d1bff35',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 20, "unit": {"Kg": 5}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('0319d5d6-2fa4-433f-be19-2a4ca6584d56', 'fa5c962a-609f-4f58-b573-25bf29e77788',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('aeec88b6-af24-4ffd-b6bf-9ae53c61f610', '219f353d-4ab4-44d5-8504-f85b906d0cbd',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('92873741-a467-4aec-b409-85bc75e06859', '52304ad3-8ba6-4654-87ec-e53635db4d7c',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 250, "unit": "Crate"}',
      'Green pepper', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('ec74b4fe-0304-4669-bb3c-56422ad0d68d', 'd80ead28-a531-459c-911b-273a97aab929',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 60, "unit": {"Kg": 10}}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('d93b8064-0761-4c1a-890f-33e0641fa743', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  '9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', '{"amount": 250, "unit": "Crate"}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),


-- Pennie Project cultivars

      ('5b98cbb8-d4bb-4516-b346-8767acc8dedf', '487fc524-47a2-426c-a289-1234041e590b',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 250, "unit": "Crate"}',
       'Juliet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('900f8270-7e46-4842-a492-53c8579cdc14', '9a48090c-caae-4c44-aeee-4dd507b6ccbe', '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 5, "unit": {"Kg": 1}}',
       'Crimson sweet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('7386f03a-a8ac-4b52-8f32-940a56eb4a4e', '63b55154-09eb-4911-b32f-632815cfb3e6',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 70, "unit": {"Kg": 10}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('d5547bbe-f8b4-49d2-ae6e-a2a0d0657537', '6ecdb665-4a50-478e-a67c-c5ee4c22b872', '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 200, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('e284c673-28a5-4054-a125-4ae1c31c8cd6', '3251249a-972b-4ae6-aeb3-12721d1bff35',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 20, "unit": {"Kg": 5}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('2c2c03df-16ca-4240-8316-392d4c796c13', 'fa5c962a-609f-4f58-b573-25bf29e77788',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('74c9f07e-544b-4b8d-8b63-47c1f6b0292f', '219f353d-4ab4-44d5-8504-f85b906d0cbd',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('7ef4b20f-ac57-4c30-a14e-ff07c27ab249', '52304ad3-8ba6-4654-87ec-e53635db4d7c',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 250, "unit": "Crate"}',
      'Green pepper', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('896e6616-e2a2-4bf4-94be-5e7258e0b018', 'd80ead28-a531-459c-911b-273a97aab929',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 60, "unit": {"Kg": 10}}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('5ad68791-4322-438a-80f6-d95b94679320', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  '1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '{"amount": 250, "unit": "Crate"}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),


-- Chicco Veg Project cultivars (not done change id first column)

      ('948b66c0-dc78-4132-9acf-1c15e3d2a7a9', '487fc524-47a2-426c-a289-1234041e590b',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 250, "unit": "Crate"}',
       'Juliet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('5eabe43e-459d-42d3-98bc-677a961ee80b', '9a48090c-caae-4c44-aeee-4dd507b6ccbe', '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 5, "unit": {"Kg": 1}}',
       'Crimson sweet', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('d072f928-bfdd-42ec-b9c0-a34cb96328c7', '63b55154-09eb-4911-b32f-632815cfb3e6',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 70, "unit": {"Kg": 10}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('07cb79cb-494c-42b8-b295-b813933078b1', '6ecdb665-4a50-478e-a67c-c5ee4c22b872', '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 200, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('97174c4c-323d-4320-a106-6709ec7a53a4', '3251249a-972b-4ae6-aeb3-12721d1bff35',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 20, "unit": {"Kg": 5}}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('3608e3ce-7191-4624-842d-397450b4e63a', 'fa5c962a-609f-4f58-b573-25bf29e77788',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

      ('c3957eb4-bb6e-43d8-adef-a547cf1feb0c', '219f353d-4ab4-44d5-8504-f85b906d0cbd',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 250, "unit": "Crate"}',
       null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('eaaf9221-259c-4483-b0fa-c17badb9fd65', '52304ad3-8ba6-4654-87ec-e53635db4d7c',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 250, "unit": "Crate"}',
      'Green pepper', null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('0deb16b2-cd0f-48f4-b455-29fda6d7cab8', 'd80ead28-a531-459c-911b-273a97aab929',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 60, "unit": {"Kg": 10}}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00'),

     ('0ad4822b-6792-4d7a-96af-88a19aa00774', '0f699e18-e1b7-4af5-8a35-497ab73a1462',  '4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '{"amount": 250, "unit": "Crate"}',
      null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false, null, null, null, '2022-09-06 10:02:25.533896846 +00:00:00');
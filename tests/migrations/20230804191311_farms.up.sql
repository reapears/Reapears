-- Add up migration script here
-- seriveces.farms


INSERT INTO services.farms(id, owner_id, name,  registered_on, deleted)
    VALUES('42b11160-96cf-4d74-bbb1-21d5a2172a8c', 'def69678-767b-4a02-b5bc-28d31152c851', 'Benson Agri Farms', '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('d53eb586-38f2-4289-a8a9-e5cf24257791', 'def69678-767b-4a02-b5bc-28d31152c851', 'Benson Avocadoes Farms', '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('99f3aa90-ed92-431d-8229-19bcaf15a8f4', '5ac29de6-95c7-45f5-b46f-cb0b8fa90d2b', 'Ammy Green Vegs', '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('bffa1215-4ce2-4df8-92d7-e6e339036dd6', '1f52b2cf-4d96-4065-bb5b-6be1b8e2dca0', 'M-cy Vegatable', '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('159f4aaa-c4bf-4045-bedc-4084f092d19b', '292a485f-a56a-4938-8f1a-bbbbbbbbbbb1', 'Pennie Project', '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('322cfb05-a099-4f46-bea1-93cbc04e4a39', '5bc31df5-81c3-45a4-94b9-89f40409ae29', 'Chicco Veg Project', '2022-09-06 10:02:25.533896846 +00:00:00', false);


-- services.locations


INSERT INTO services.locations(id, farm_id, place_name, country_id, region_id, description, coords, created_at, deleted)
    VALUES('65e67229-989c-46fa-ab54-0d26ff2d8a18', '42b11160-96cf-4d74-bbb1-21d5a2172a8c', 'Ohaingu', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d1-7b03-8104-e76550250c0a', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('b9ad294c-37b5-457f-9ddb-6388cb156f74', '99f3aa90-ed92-431d-8229-19bcaf15a8f4', 'Oshihenye', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d0-7570-895a-7102c6536b63', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('9b6e6c71-8bfb-40e2-bdf7-c635b0ae63b9', 'bffa1215-4ce2-4df8-92d7-e6e339036dd6', 'Oshihenye', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d0-7570-895a-7102c6536b63', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('1c70a55e-47ec-464a-bf1c-fbfc52edccdc', '159f4aaa-c4bf-4045-bedc-4084f092d19b', 'Omindamba', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d0-7570-895a-7102c6536b63', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false),
          ('4c0a38b1-2c08-4d6c-b2c7-cc709defc0e1', '322cfb05-a099-4f46-bea1-93cbc04e4a39', 'Okapanda', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d0-7570-895a-7102c6536b63', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false),


-- other pennies location farms
          ('1465c1b8-6fb3-462b-afc0-c4b0904a64d0', '159f4aaa-c4bf-4045-bedc-4084f092d19b', 'Epalela', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d0-7570-895a-7102c6536b63', null, null, '2022-09-26 10:02:25.533896846 +00:00:00', false),

-- onother bensons farm 
          ('5b793e9f-94f1-4f8c-96d5-8aef167c461c', 'd53eb586-38f2-4289-a8a9-e5cf24257791', 'Eenhanna', '0189c073-51d1-77e9-ae60-f506ede3e22e', '0189c073-51d1-7b03-8104-e76550250c0a', null, null, '2022-09-06 10:02:25.533896846 +00:00:00', false);


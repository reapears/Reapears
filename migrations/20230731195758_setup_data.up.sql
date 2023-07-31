-- Add up migration script here

-- services.cultivar_categories


INSERT INTO services.cultivar_categories(id, name)
    VALUES('d0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Vegetable'),
        ('deeef1ff-9165-4af1-8885-ab326e873969', 'Fruit'),
        ('c6b743bc-0961-4607-bcc1-f4fd6f81f524', 'Grain'),
        ('c64eac22-5069-46ea-bb83-b071dbe72e3b', 'Tuber');



-- services.cultivars


INSERT INTO services.cultivars(id, category_id, name)
    VALUES('487fc524-47a2-426c-a289-1234041e590b', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Tomatoes'),
          ('9a48090c-caae-4c44-aeee-4dd507b6ccbe', 'deeef1ff-9165-4af1-8885-ab326e873969', 'Watermelons'),
          ('63b55154-09eb-4911-b32f-632815cfb3e6', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Butternuts'),
          ('6ecdb665-4a50-478e-a67c-c5ee4c22b872', 'deeef1ff-9165-4af1-8885-ab326e873969', 'Mangoes'),
          ('3251249a-972b-4ae6-aeb3-12721d1bff35', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Cabbages'),
          ('fa5c962a-609f-4f58-b573-25bf29e77788', 'c64eac22-5069-46ea-bb83-b071dbe72e3b', 'Sweet potatoes'),
          ('219f353d-4ab4-44d5-8504-f85b906d0cbd', 'c6b743bc-0961-4607-bcc1-f4fd6f81f524', 'Maize'),
          ('52304ad3-8ba6-4654-87ec-e53635db4d7c', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Bell Peppers'),
          ('d80ead28-a531-459c-911b-273a97aab929', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Onions'),
          ('0f699e18-e1b7-4af5-8a35-497ab73a1462', 'd0a73812-54e0-4cc0-837e-0eb1c7432a2e', 'Carrots');



-- services.countries


INSERT INTO services.countries(id, name)
    VALUES('608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Namibia');



-- services.regions


INSERT INTO services.regions(id, country_id, name)
    VALUES('c213e06b-db7f-4a7e-90fd-f8b860037a91', '608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Omusaati'),
          ('36e63281-efe7-427b-bab0-33aab613b498', '608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Ohangwena'),
          ('bc66089b-fd8e-4d10-9a70-cc2305872198', '608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Capriv'),
          ('80e7d4b9-f335-4763-b94a-2886b2135677', '608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Kavango'),
          ('983708b2-44d6-48d9-beb1-466a956e1b7f', '608de95c-beb1-4b2b-b63e-ea9787ed60f4', 'Hardap');



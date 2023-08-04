-- Add up migration script here

-- services.cultivar_categories

INSERT INTO services.cultivar_categories(id, name)
    VALUES('0189c073-51d3-7215-85f2-91579af8024b', 'Vegetable'),
        ('0189c073-51d3-71ad-9a91-323fe86536dc', 'Fruit'),
        -- ('0189c073-51d3-7f86-9ab4-c984defe40a8', 'Grain'),
        ('0189c073-51d3-76bb-8d45-dee12d7cde30', 'Tuber'),
        ('0189c073-51d3-7728-8fc4-6915fa064e27', 'Squash'),
        ('0189c073-51d3-7a85-945f-00ea022e74dc', 'Roots'),
        ('0189c073-51d4-733b-9663-7b7dbfc8383b', 'Grain');

-- services.cultivars

INSERT INTO services.cultivars(id, category_id, name)
VALUES('0189c073-51d4-7027-a6dd-44f56b5ad469','0189c073-51d3-7215-85f2-91579af8024b', 'Maize'),
    ('0189c073-51cd-7055-b6b7-68ba7d936168', '0189c073-51d3-7215-85f2-91579af8024b', 'Sweet corn'),
    ('0189c073-51cd-766e-a3da-1a7462937354', '0189c073-51d3-7215-85f2-91579af8024b', 'Onion'),
    ('0189c073-51cd-7996-987e-862693ff820d', '0189c073-51d3-7215-85f2-91579af8024b', 'Tomato'),
    ('0189c073-51cd-7a0e-8e27-d3b0333ebdad', '0189c073-51d3-7215-85f2-91579af8024b', 'Cabbage'),
    ('0189c073-51cd-70e0-b512-a6af230d99ee', '0189c073-51d3-7215-85f2-91579af8024b', 'Spinach'),
    ('0189c073-51cd-7107-a6e4-086b2099dd77', '0189c073-51d3-7215-85f2-91579af8024b', 'Bell pepper'),
    ('0189c073-51cd-7a01-9d6a-6869c69776ee', '0189c073-51d3-7728-8fc4-6915fa064e27', 'Butternut'),
    ('0189c073-51cd-708c-b39b-2de20bac3b74', '0189c073-51d3-76bb-8d45-dee12d7cde30', 'Sweet potato'),
    ('0189c073-51ce-7991-bacb-b4001ae9f959', '0189c073-51d3-76bb-8d45-dee12d7cde30', 'Potato'),
    ('0189c073-51ce-795d-a677-f51878b3e6eb', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Watermelon'),
    ('0189c073-51ce-7856-b3c6-ee2895d0d997', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Sweet melon'),
    ('0189c073-51ce-78fb-9b86-d784fd49385e', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Mango'),
    ('0189c073-51ce-7195-874f-0cc5fed6b744', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Lemon'),
    ('0189c073-51ce-7024-88dd-b6ba9c246dfa', '0189c073-51d3-7215-85f2-91579af8024b', 'Chili pepper'),
    ('0189c073-51ce-7a2f-be2a-9c28933df93c', '0189c073-51d4-733b-9663-7b7dbfc8383b', 'Wheat'),
    ('0189c073-51cf-795b-b111-2f8647dbb50a', '0189c073-51d3-7215-85f2-91579af8024b', 'Pumpkin'),
    ('0189c073-51cf-7a36-addd-3a1773374aaa', '0189c073-51d3-7a85-945f-00ea022e74dc', 'Carrot'),
    ('0189c073-51cf-7110-9e9c-de268ea06edc', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Sugarcane'),
    ('0189c073-51cf-795c-8e10-85cd8b5885ab', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Papaya'),
    ('0189c073-51cf-7fad-bcf9-e2edf3d3da99', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Dates'),
    ('0189c073-51cf-7bdd-9e02-bd78306394ea', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Strawberry'),
    ('0189c073-51d0-7dda-ab13-e296b88726ec', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Banana'),
    ('0189c073-51d0-731b-a78d-cc0f7e25f3d4', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Grape'),
    ('0189c073-51d0-71a7-b684-93ace15a5e26', '0189c073-51d3-71ad-9a91-323fe86536dc', 'Dried grape');

-- services.countries


INSERT INTO services.countries(id, name)
    VALUES('0189c073-51d1-77e9-ae60-f506ede3e22e', 'Namibia');


-- services.regions


INSERT INTO services.regions(id, country_id, name)
VALUES('0189c073-51d0-7f61-8fff-e14088710e6f', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Kunene'),
      ('0189c073-51d0-7570-895a-7102c6536b63', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Omusati'),
      ('0189c073-51d1-714c-aed1-2d6ad763b1ec', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Oshana'),
      ('0189c073-51d1-7b03-8104-e76550250c0a', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Ohangwena'),
      ('0189c073-51d1-76d4-bfdc-e6b23a1812f3', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Oshikoto'),
      ('0189c073-51d1-7663-9ab9-76f9059555f1', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Kavango East'),
      ('0189c073-51d1-706d-953e-3f2fa546468f', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Zambezi'),
      ('0189c073-51d2-71e3-9103-20dd6f042ced', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Erongo'),
      ('0189c073-51d2-7a4e-8d92-3a5892a42470', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Otjozondjupa'),
      ('0189c073-51d2-782c-b478-8162580db4c7', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Omaheke'),
      ('0189c073-51d2-7be2-adbe-9c0b04e4e3fb', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Khomas'),
      ('0189c073-51d2-7379-960e-46f99509f124', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Hardap'),
      ('0189c073-51d2-7d7b-a9e0-8bcb138908d7', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'ǁKaras'),
      ('0189c073-51d2-7bbd-bcd4-38b9201f44a6', '0189c073-51d1-77e9-ae60-f506ede3e22e', 'Kavango West');

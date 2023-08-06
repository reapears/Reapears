-- Add down migration script here

DELETE FROM services.cultivar_categories;

DELETE FROM services.cultivars;

DELETE FROM services.regions;

DELETE FROM services.countries;

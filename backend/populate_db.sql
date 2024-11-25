-- Populate the tenants table with specified IDs
INSERT INTO tenants (id, name, age, image, burn_count, dishwasher_count, favorite_quote)
VALUES
  (63, 'Mr. Horse', 24, 'horse.png', 0, 0, 'quote 1'),
  (64, 'Mr. Cat', 24, 'cat.png', 0, 0, 'quote 2'),
  (65, 'Mr. Dog', 24, 'dog.png', 0, 0, 'quote 3'),
  (66, 'Mr. Goat', 24, 'goat.png', 0, 0, 'quote 4'),
  (67, 'Mrs. Cow', 24, 'cow.png', 0, 0, 'quote 5'),
  (68, 'Mr. Falcon', 24, 'falcon.png', 0, 0, 'quote 6');

-- Populate the burn table using the correct tenant IDs
INSERT INTO burn (reason, receiver_id, giver_id, created_at)
VALUES
  ('Forgot to clean up after dinner', 63, 64, '2024-11-01 14:30:00'),
  ('Did not take out the trash', 64, 65, '2024-11-02 09:15:00'),
  ('Left dirty dishes in the sink', 65, 66, '2024-11-03 19:00:00'),
  ('Forgot to vacuum the living room', 66, 67, '2024-11-04 12:45:00'),
  ('Left the door unlocked', 67, 68, '2024-11-05 08:20:00'),
  ('Did not refill the toilet paper', 68, 63, '2024-11-06 16:10:00'),
  ('Forgot to clean the bathroom', 63, 65, '2024-11-07 10:00:00'),
  ('Spilled coffee on the table and left it', 65, 67, '2024-11-08 13:50:00'),
  ('Did not replace empty milk carton in fridge', 67, 66, '2024-11-09 17:30:00'),
  ('Burnt toast and set off fire alarm', 64, 68, '2024-11-10 07:45:00');

-- Set the tenants_id_seq to the max id to ensure future inserts use the correct sequence
SELECT setval('tenants_id_seq', (SELECT MAX(id) FROM tenants));

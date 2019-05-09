-- *****************************************************************************

insert into dialogue.t_group(f_group_id, f_description, f_allowed, f_moderated)
  values('local.test', 'Local testing group.', true, false);
insert into dialogue.t_group(f_group_id, f_description, f_allowed, f_moderated)
  values('local.test2', 'Secondary local testing group.', false, false);
insert into dialogue.t_group(f_group_id, f_description, f_allowed, f_moderated)
  values('local.dialogue.announce', 'Announcement group for the Dialogue NNTP server.', true, true);
insert into dialogue.t_group(f_group_id, f_description, f_allowed, f_moderated)
  values('local.dialogue.general', 'General discussion group about the Dialogue NNTP server.', true, false);

-- *****************************************************************************

insert into dialogue.t_article(f_message_id, f_body, f_path, f_from, f_subject, f_date)
  values(
    'test-message-1@development.dialogue.richard70.nl',
    'This is test message 1.',
    'development.dialogue.richard70.nl!not-for-mail',
    'Test <no-mail@test.test>',
    'Test 1',
    now()
  );

insert into dialogue.t_group_article(f_group_id, f_message_id, f_number)
  values('local.test', 'test-message-1@development.dialogue.richard70.nl', 1);

update dialogue.t_group set f_sequence = 1 where f_group_id = 'local.test';

-- *****************************************************************************

insert into dialogue.t_article(f_message_id, f_body, f_path, f_from, f_subject, f_date)
  values(
    'test-message-2@development.dialogue.richard70.nl',
    'This is test message 2.',
    'development.dialogue.richard70.nl!not-for-mail',
    'Test <no-mail@test.test>',
    'Test 2',
    now()
  );

insert into dialogue.t_group_article(f_group_id, f_message_id, f_number)
  values('local.test', 'test-message-2@development.dialogue.richard70.nl', 2);

insert into dialogue.t_group_article(f_group_id, f_message_id, f_number)
  values('local.test2', 'test-message-2@development.dialogue.richard70.nl', 1);

update dialogue.t_group set f_sequence = 2 where f_group_id = 'local.test';
update dialogue.t_group set f_sequence = 1 where f_group_id = 'local.test2';

-- *****************************************************************************

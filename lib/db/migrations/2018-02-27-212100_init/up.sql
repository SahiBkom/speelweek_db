-- Your SQL goes here

CREATE TABLE `user` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(255) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `login_at` datetime DEFAULT NULL,
  `voorletters` varchar(255) DEFAULT NULL,
  `voornaam` varchar(255) DEFAULT NULL,
  `tussenvoegsel` varchar(255) DEFAULT NULL,
  `achternaam` varchar(255) DEFAULT NULL,
  `straatnaam` varchar(255) DEFAULT NULL,
  `huisnummer` varchar(255) DEFAULT NULL,
  `toevoeging_huisnummer` varchar(255) DEFAULT NULL,
  `postcode` varchar(255) DEFAULT NULL,
  `woonplaats` varchar(255) DEFAULT NULL,
  `telefoon_nummer` varchar(255) DEFAULT NULL,
  `mobiele_nummer` varchar(255) DEFAULT NULL,
  `e_mail_adres` varchar(255) DEFAULT NULL,
  `e_mail_toestemming` tinyint(1) DEFAULT '0',
  `role` bigint(20) unsigned DEFAULT NULL,
  `geboortedatum` date DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_user_username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8

-- remove
CREATE TABLE `tasks` (
  `id` int(11) DEFAULT NULL,
  `description` varchar(256) DEFAULT NULL,
  `completed` tinyint(1) NOT NULL DEFAULT '0',
) ENGINE=InnoDB DEFAULT CHARSET=utf8

CREATE TABLE `team` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `naam` varchar(255) NOT NULL,
  `omschrijving` varchar(255) NOT NULL,
  `voorbereiding` tinyint(1) NOT NULL DEFAULT '0',
  `middagprogramma` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8

CREATE TABLE `team_vrijwilliger` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `team_id` int(10) unsigned NOT NULL,
  `user_id` int(10) unsigned NOT NULL,
  `programma` tinyint(1) NOT NULL DEFAULT '0',
  `voorbereiding` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `ix_team_vrijwilliger_team_id` (`team_id`),
  KEY `ix_team_vrijwilliger_vrijwilliger_id` (`user_id`),
  CONSTRAINT `team_vrijwilliger_team_FK` FOREIGN KEY (`id`) REFERENCES `team` (`id`),
  CONSTRAINT `team_vrijwilliger_user_FK` FOREIGN KEY (`id`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8


INSERT INTO `team` (`id`, `naam`, `omschrijving`, `voorbereiding`, `middagprogramma`, `created_at`, `updated_at`) VALUES
(1, 'baktent', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(2, 'bestuur', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(3, 'decor', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(4, 'beurs', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(5, 'casino', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(6, 'FANTA-krant', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(7, 'muziektent', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(8, 'danstent', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(9, 'communicatie', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(10, 'crea', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(11, 'deelnemers', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(12, 'EHBO', '', 0, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(13, 'FANTA-kermis', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(14, 'financiën', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(15, 'FLEX-pool', '(limobar / filmtent / skelterbaan / leestent)', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(16, 'foerage & schoonmaak', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(17, 'fotostudio', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(18, 'huttenbouw', '', 0, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(19, 'ict', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(20, 'infra', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(21, 'kabelbaan & klimwand', '', 0, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(22, 'laboratorium', '(proefjes)', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(23, 'licht en geluid', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(24, 'logistiek', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(25, 'materiaaltent', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(26, 'op- & afbouw', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(27, 'outdoor', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(28, 'ontwikkeling', '(nieuwe onderdelen)', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(29, 'postkantoor', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(30, 'programma', '(ochtendprogramma)', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(31, 'podiumprogramma', '', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(32, 'schoonheidssalon', '(schmink)', 0, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(33, 'sponsoring', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(34, 'sponsorcommissie', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(35, 'thema', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(36, 'veiligheid', '', 1, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(37, 'vrijwilligers', '', 0, 0, '2017-05-22 19:17:45', '2017-05-22 19:17:45'),
(38, 'water & fun', '(waterprogramma / springkussens / trampolines)', 1, 1, '2017-05-22 19:17:45', '2017-05-22 19:17:45');


CREATE TABLE `property` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tag` varchar(255) NOT NULL,
  `value` longtext NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_tag` (`tag`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8

CREATE TABLE `emailtemplate` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tag` varchar(255) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `text` longtext NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_tag` (`tag`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
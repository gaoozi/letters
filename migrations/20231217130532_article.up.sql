-- Add table article
CREATE TABLE IF NOT EXISTS article (
  id INT NOT NULL AUTO_INCREMENT,
  title VARCHAR(256) NOT NULL,
  slug VARCHAR(128),
  content TEXT NOT NULL,
  summary VARCHAR(256),
  cover VARCHAR(64),
  status TINYINT(8) NOT NULL,
  password VARCHAR(32),
  read_count INT NOT NULL DEFAULT 0,
  like_count INT NOT NULL DEFAULT 0,
  is_top TINYINT(1) NOT NULL DEFAULT 0,
  category_id INT NOT NULL,
  author_id INT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  deleted_at DATETIME,
  PRIMARY KEY (`id`),
  CONSTRAINT `article_author_id` FOREIGN KEY (`author_id`) REFERENCES `user` (`id`),
  CONSTRAINT `article_category_id` FOREIGN KEY (`category_id`) REFERENCES `category` (`id`)
);

-- Add table article_tag
CREATE TABLE IF NOT EXISTS article_tag (
  id INT NOT NULL AUTO_INCREMENT,
  article_id INT NOT NULL,
  tag_id INT NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `at_article_id` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`),
  CONSTRAINT `at_tag_id` FOREIGN KEY (`tag_id`) REFERENCES `tag` (`id`)
);

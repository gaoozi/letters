SELECT
    `article`.`id`,
    `article`.`title`,
    `article`.`slug`,
    `article`.`cover`,
    `article`.`content`,
    `article`.`summary`,
    `article`.`password_hash`,
    `article`.`source`,
    `article`.`source_url`,
    `article`.`topping`,
    `article`.`status`,
    `article`.`created_at`,
    `article`.`updated_at`,
    `article`.`deleted_at`,
    `article`.`user_id`,
    `article`.`category_id`
FROM
    `article`
    LEFT JOIN `article` ON `user`.`id` = `article`.`user_id`
    LEFT JOIN `article` ON `category`.`id` = `article`.`category_id`
    LEFT JOIN `article` ON `article_tag`.`article_id` = `article`.`id`
    LEFT JOIN `article_tag` ON `tag`.`id` = `article_tag`.`tag_id`
GROUP BY
    `article`.`id`
HAVING
    `article`.`id` = 2
SELECT
    `article`.`id`,
    `article`.`title`,
    `article`.`slug`,
    `article`.`cover`,
    `article`.`content`,
    `article`.`summary`,
    `article`.`password_hash`,
    `article`.`source`,
    `article`.`source_url`,
    `article`.`topping`,
    `article`.`status`,
    `article`.`created_at`,
    `article`.`updated_at`,
    `article`.`deleted_at`,
    `article`.`user_id`,
    `article`.`category_id`
FROM
    `article`
    LEFT JOIN `user` ON `article`.`user_id` = `user`.`id`
    LEFT JOIN `category` ON `article`.`category_id` = `category`.`id`
    LEFT JOIN `article_tag` ON `article`.`id` = `article_tag`.`article_id`
    LEFT JOIN `article_tag` ON `tag`.`id` = `article_tag`.`tag_id`
WHERE
    `article`.`id` = 2
GROUP BY
    `article`.`id`
HAVING
    `article`.`id` = 2


SELECT
    `article`.`id`,
    `article`.`title`,
    `article`.`slug`,
    `article`.`cover`,
    `article`.`content`,
    `article`.`summary`,
    `article`.`password_hash`,
    `article`.`source`,
    `article`.`source_url`,
    `article`.`topping`,
    `article`.`status`,
    `article`.`created_at`,
    `article`.`updated_at`,
    `article`.`deleted_at`,
    `article`.`user_id`,
    `article`.`category_id`
FROM
    `article`
    LEFT JOIN `user` ON `article`.`user_id` = `user`.`id`
    LEFT JOIN `category` ON `article`.`category_id` = `category`.`id`
    LEFT JOIN `article_tag` ON `article`.`id` = `article_tag`.`article_id`
    LEFT JOIN `tag` ON `article_tag`.`tag_id` = `tag`.`id`
GROUP BY
    `article`.`id`
HAVING
    `article`.`id` = 2

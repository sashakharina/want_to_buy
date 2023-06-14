ALTER TABLE lists 
ADD CONSTRAINT unique_list_name_for_user 
UNIQUE (user_id, name);
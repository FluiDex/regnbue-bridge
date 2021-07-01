CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_time = CURRENT_TIMESTAMP; 
   RETURN NEW;
END;
$$ language 'plpgsql';

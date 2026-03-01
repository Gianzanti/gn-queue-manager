-- Migration file to add new fields to the visitors table

-- Add new fields observations and confirmVisit to the visitors table
ALTER TABLE visitors
ADD COLUMN observations TEXT DEFAULT '',
ADD COLUMN confirm_visit BOOLEAN DEFAULT FALSE;

-- Update existing records to set default values
UPDATE visitors
SET observations = '',
    confirm_visit = FALSE;

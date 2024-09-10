-- Migration file to add new fields to the visitors table

-- Add new fields observations and confirmVisit to the visitors table
ALTER TABLE visitors
ADD COLUMN state TEXT DEFAULT '',
ADD COLUMN job TEXT DEFAULT '';

-- Update existing records to set default values
UPDATE visitors
SET state = '',
    job = '';

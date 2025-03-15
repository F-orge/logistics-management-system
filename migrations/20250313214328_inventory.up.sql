-- Add up migration script here
CREATE TABLE
  logistics.package (
    id uuid not null primary key default gen_random_uuid (),
    name text not null,
    arrive_time timestamp not null,
    cargo_type varchar not null check (cargo_type in ('perishable', 'non-perishable')),
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );

CREATE TABLE
  logistics.shipment (
    id uuid not null primary key default gen_random_uuid (),
    from_address text not null,
    to_address text not null,
    packages uuid[] not null,
    created timestamp not null default current_timestamp,
    updated timestamp not null default current_timestamp
  );
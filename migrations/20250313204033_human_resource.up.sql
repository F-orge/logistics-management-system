-- Add up migration script here
CREATE TABLE
  logistics.department (
    department_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    department_name VARCHAR(255) UNIQUE NOT NULL,
    manager_id UUID NULL
  );

CREATE TABLE
  logistics.position (
    position_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    position_name VARCHAR(255) UNIQUE NOT NULL,
    job_description TEXT NOT NULL
  );

CREATE TABLE
  logistics.employee (
    employee_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20) UNIQUE,
    address TEXT,
    hire_date DATE NOT NULL,
    department_id UUID NOT NULL,
    position_id UUID NOT NULL,
    supervisor_id UUID NULL,
    FOREIGN KEY (department_id) REFERENCES logistics.department (department_id) ON DELETE SET NULL,
    FOREIGN KEY (position_id) REFERENCES logistics.position (position_id) ON DELETE SET NULL,
    FOREIGN KEY (supervisor_id) REFERENCES logistics.employee (employee_id) ON DELETE SET NULL
  );

CREATE TABLE
  logistics.task (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    task_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    assigned_by UUID NOT NULL,
    assigned_to UUID NOT NULL,
    start_date DATE NOT NULL,
    due_date DATE NOT NULL CHECK (due_date >= start_date),
    priority VARCHAR(10) NOT NULL CHECK (priority IN ('Low', 'Medium', 'High')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('Pending', 'In Progress', 'Completed')),
    FOREIGN KEY (assigned_by) REFERENCES logistics.employee (employee_id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES logistics.employee (employee_id) ON DELETE CASCADE
  );
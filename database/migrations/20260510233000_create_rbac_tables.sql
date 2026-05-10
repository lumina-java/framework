-- Migration: Create RBAC Tables
-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id         BIGINT AUTO_INCREMENT PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    slug       VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);

-- Permissions table
CREATE TABLE IF NOT EXISTS permissions (
    id         BIGINT AUTO_INCREMENT PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    slug       VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);

-- Role-Permission pivot table
CREATE TABLE IF NOT EXISTS role_permission (
    role_id       BIGINT NOT NULL,
    permission_id BIGINT NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- User-Role pivot table
CREATE TABLE IF NOT EXISTS user_role (
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

-- Initial Roles
INSERT IGNORE INTO roles (name, slug) VALUES ('Administrator', 'admin');
INSERT IGNORE INTO roles (name, slug) VALUES ('User', 'user');

-- Initial Permissions
INSERT IGNORE INTO permissions (name, slug) VALUES ('View Dashboard', 'view_dashboard');
INSERT IGNORE INTO permissions (name, slug) VALUES ('Manage Users', 'manage_users');
INSERT IGNORE INTO permissions (name, slug) VALUES ('Manage Settings', 'manage_settings');

-- Assign all permissions to admin
INSERT IGNORE INTO role_permission (role_id, permission_id) 
SELECT r.id, p.id FROM roles r, permissions p WHERE r.slug = 'admin';

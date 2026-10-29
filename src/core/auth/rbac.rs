use serde::{Deserialize, Serialize};
use super::AuthUser;

/// Trait untuk memeriksa hak akses dan peran (RBAC) pada entitas pengguna.
pub trait HasRolesAndPermissions {
    /// Cek apakah pengguna memiliki izin tertentu.
    /// Pengguna dengan role `admin` atau `superadmin` otomatis memiliki semua izin.
    fn can(&self, permission: &str) -> bool;

    /// Kebalikan dari `can`: cek apakah pengguna TIDAK memiliki izin.
    fn cannot(&self, permission: &str) -> bool {
        !self.can(permission)
    }

    /// Cek apakah pengguna memiliki peran (role) tertentu.
    fn has_role(&self, role: &str) -> bool;

    /// Cek apakah pengguna memiliki salah satu dari daftar peran.
    fn has_any_role(&self, roles: &[&str]) -> bool;

    /// Cek apakah pengguna memiliki semua peran dalam daftar.
    fn has_all_roles(&self, roles: &[&str]) -> bool;

    /// Cek apakah pengguna memiliki salah satu dari daftar izin.
    fn can_any(&self, permissions: &[&str]) -> bool;
}

impl HasRolesAndPermissions for AuthUser {
    fn can(&self, permission: &str) -> bool {
        if self.role == "superadmin" || self.role == "admin" {
            return true;
        }
        self.permissions.iter().any(|p| p == permission || p == "*")
    }

    fn has_role(&self, role: &str) -> bool {
        if self.role == "superadmin" {
            return true;
        }
        self.role == role
    }

    fn has_any_role(&self, roles: &[&str]) -> bool {
        if self.role == "superadmin" {
            return true;
        }
        roles.iter().any(|&r| self.has_role(r))
    }

    fn has_all_roles(&self, roles: &[&str]) -> bool {
        if self.role == "superadmin" {
            return true;
        }
        roles.iter().all(|&r| self.has_role(r))
    }

    fn can_any(&self, permissions: &[&str]) -> bool {
        if self.role == "superadmin" || self.role == "admin" {
            return true;
        }
        permissions.iter().any(|&p| self.can(p))
    }
}

/// Representasi Role dalam sistem RBAC.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

/// Representasi Permission dalam sistem RBAC.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}


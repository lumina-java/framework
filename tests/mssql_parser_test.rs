#[cfg(feature = "mssql")]
#[test]
fn test_universal_mssql_parser() {
    use lumina::database::connection::DatabasePool;

    // 1. Format URL Standard (seperti di dokumentasi lumina sebelumnya)
    let url1 = "sqlserver://sa:YourPassword123@localhost:1433?database=lumina_db&trustServerCertificate=true";
    let cfg1 = DatabasePool::parse_mssql_config(url1);
    assert!(cfg1.is_ok(), "Gagal parsing url1: {:?}", cfg1.err());

    // 2. Format URL dengan Named Instance dan query database (kasus kantor user)
    let url2 = "sqlserver://sa:123@192.168.10.29/SQL2012?database=DBSIMRS&trustServerCertificate=true";
    let cfg2 = DatabasePool::parse_mssql_config(url2);
    assert!(cfg2.is_ok(), "Gagal parsing url2: {:?}", cfg2.err());

    // 3. Format URL mssql://
    let url3 = "mssql://sa:123@192.168.10.29:1433/DBSIMRS?trustServerCertificate=true";
    let cfg3 = DatabasePool::parse_mssql_config(url3);
    assert!(cfg3.is_ok(), "Gagal parsing url3: {:?}", cfg3.err());

    // 4. Format JDBC langsung dengan Named Instance double backslash
    let jdbc1 = "jdbc:sqlserver://192.168.10.29\\SQL2012;user=sa;password=123;database=DBSIMRS;trustServerCertificate=true";
    let cfg4 = DatabasePool::parse_mssql_config(jdbc1);
    assert!(cfg4.is_ok(), "Gagal parsing jdbc1: {:?}", cfg4.err());

    // 5. Format JDBC langsung dengan port
    let jdbc2 = "jdbc:sqlserver://192.168.10.29:1433;user=sa;password=123;database=DBSIMRS;trustServerCertificate=true";
    let cfg5 = DatabasePool::parse_mssql_config(jdbc2);
    assert!(cfg5.is_ok(), "Gagal parsing jdbc2: {:?}", cfg5.err());

    // 6. Format ADO Connection String
    let ado1 = "Server=192.168.10.29,1433;Database=DBSIMRS;User Id=sa;Password=123;TrustServerCertificate=true;";
    let cfg6 = DatabasePool::parse_mssql_config(ado1);
    assert!(cfg6.is_ok(), "Gagal parsing ado1: {:?}", cfg6.err());

    println!("✅ Semua 6 variasi connection string MSSQL berhasil di-parse sempurna oleh Smart Parser!");
}


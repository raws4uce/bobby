package _0.localhost.kinesthesia;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;


import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Repository;
import org.springframework.util.Assert;



@Repository
public class ResRepository {
	
	private final Logger log = LoggerFactory.getLogger(KinesthesiaApplication.class);
	private final JdbcClient jdbc;
	List<Result> list = new ArrayList<>();
	
    public ResRepository(JdbcClient jdbcClient) {
        this.jdbc = jdbcClient;
    }

    public List<Result> findAll() {
        return jdbc.sql("select * from Results")
                .query(Result.class)
                .list();
    }

    public Optional<Result> findById(Integer id) {
        return jdbc.sql("SELECT id,name, val FROM Results WHERE id = :id" )
                .param("id", id)
                .query(Result.class)
                .optional();
    }

    public void create(Result Result) {
        var updated = jdbc.sql("INSERT INTO Results(id,name, val) values(?,?,?)")
                .params(List.of(Result.id(),Result.name(),Result.val().toString()))
                .update();

        Assert.state(updated == 1, "Failed to create Results" + Result.name());
    }

    public void update(Result Result, Integer id) {
        var updated = jdbc.sql("update Results set name = ?, val = ? where id = ?")
                .params(List.of(Result.name(),Result.val().toString(), id))
                .update();

        Assert.state(updated == 1, "Failed to update Results " + Result.name());
    }

    public void delete(Integer id) {
        var updated = jdbc.sql("delete from Results where id = :id")
                .param("id", id)
                .update();

        Assert.state(updated == 1, "Failed to delete Results" + id);
    }
    
    public int count() {
        return jdbc.sql("select * from Results").query().listOfRows().size();
    }
    
    public void saveAll(List<Result> runs) {
        runs.stream().forEach(this::create);
    }
    
}

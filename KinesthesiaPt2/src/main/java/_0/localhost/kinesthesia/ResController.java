package _0.localhost.kinesthesia;

import java.util.List;
import java.util.Optional;

import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;

import jakarta.validation.Valid;

@RestController
@RequestMapping("/api")
public class ResController {
	
	private final ResRepository repo;
	
	ResController(ResRepository repo){
		this.repo = repo;
	}
	
	@GetMapping("/val")
	List<Result> findAll(){
		return repo.findAll();
	}
	
	@GetMapping("/{id}")
	Result findById(@Valid @PathVariable Integer id) {
		Optional<Result> res = repo.findById(id);
		if(res.isEmpty()) {
			throw new ResultNotFoundException();
		}
		return res.get();
	}
	
	//post
	@ResponseStatus(HttpStatus.CREATED)
	@PostMapping("")
	void create(@Valid @RequestBody Result res) {
		repo.create(res);
	}
	
	//put
	@ResponseStatus(HttpStatus.NO_CONTENT)
	@PutMapping("/{id}")
	void update(@Valid @RequestBody Result res, @PathVariable Integer id) {
		repo.update(res, id);
	}
	//delete}
	@ResponseStatus(HttpStatus.NO_CONTENT)
	@DeleteMapping("/{id}")
	void delete(@Valid @PathVariable Integer id) {
		repo.delete(id);
	}
}

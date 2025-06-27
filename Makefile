start-jeager:
	docker run -d --name jaeger \
  -p 16686:16686 \
  -p 14268:14268 \
  jaegertracing/all-in-one:latest

test-jeager:
	curl http://localhost:16686

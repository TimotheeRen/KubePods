dev:
	k3d cluster create --config k3d/dev-env.yaml \
        --port "8080:30080@loadbalancer" \
        --port "8443:30443@loadbalancer"
	mkdir -p ~/.kube
	k3d kubeconfig get KubePods > ~/.kube/config
	helm install flux-operator oci://ghcr.io/controlplaneio-fluxcd/charts/flux-operator \
	  --namespace flux-system \
	  --create-namespace
	kubectl apply -f flux-dev.yaml

delete:
	k3d cluster delete KubePods

forward:
	kubectl port-forward svc/users-postgres-cluster-rw 5432:5432 &
	kubectl port-forward svc/desktops-postgres-cluster-rw 5433:5432 &

show-passwords:
	@echo "users-postgres-cluster-app: $$(kubectl get secret users-postgres-cluster-app -o jsonpath='{.data.password}' | base64 -d)"
	@echo "desktops-postgres-cluster-app: $$(kubectl get secret desktops-postgres-cluster-app -o jsonpath='{.data.password}' | base64 -d)"

attach:
	mkdir ~/.kube
	sudo k3d kubeconfig get KubePods > ~/.kube/config

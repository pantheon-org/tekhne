#!/usr/bin/env bash
# shell: bash
# Check for required validation tools and provide installation instructions

set -e

echo "Checking for Kubernetes YAML validation tools..."
echo

MISSING_TOOLS=()

# Check for yamllint
if ! command -v yamllint &> /dev/null; then
    echo "❌ yamllint not found"
    MISSING_TOOLS+=("yamllint")
else
    echo "✅ yamllint found: $(yamllint --version)"
fi

# Check for kubeconform
if ! command -v kubeconform &> /dev/null; then
    echo "❌ kubeconform not found"
    MISSING_TOOLS+=("kubeconform")
else
    echo "✅ kubeconform found: $(kubeconform -v)"
fi

# Check for kubectl
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found"
    MISSING_TOOLS+=("kubectl")
else
    echo "✅ kubectl found: $(kubectl version --client 2>/dev/null | head -1)"
fi

# Check for yq (optional but helpful)
if ! command -v yq &> /dev/null; then
    echo "⚠️  yq not found (optional, but helpful for YAML manipulation)"
else
    echo "✅ yq found: $(yq --version)"
fi

echo

if [ ${#MISSING_TOOLS[@]} -eq 0 ]; then
    echo "✅ All required tools are installed!"
    exit 0
else
    echo "❌ Missing tools: ${MISSING_TOOLS[*]}"
    echo
    echo "Installation instructions:"
    echo

    for tool in "${MISSING_TOOLS[@]}"; do
        case $tool in
            yamllint)
                echo "📦 yamllint:"
                echo "  macOS:   brew install yamllint"
                echo "  Linux:   pip install yamllint"
                echo "  Ubuntu:  apt-get install yamllint"
                echo
                ;;
            kubeconform)
                echo "📦 kubeconform:"
                echo "  macOS:   brew install kubeconform"
                echo "  Linux:   Download from https://github.com/yannh/kubeconform/releases"
                echo "  Or use:  go install github.com/yannh/kubeconform/cmd/kubeconform@latest"
                echo
                ;;
            kubectl)
                echo "📦 kubectl:"
                echo "  macOS:   brew install kubectl"
                echo "  Linux:   https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/"
                echo "  Or use:  curl -LO https://dl.k8s.io/release/\$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/\$(uname -s | tr '[:upper:]' '[:lower:]')/\$(uname -m)/kubectl"
                echo
                ;;
        esac
    done

    exit 1
fi

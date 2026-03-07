# Configuración específica para este proyecto
$env:ANTHROPIC_BASE_URL="https://api.minimax.io/anthropic"
$env:ANTHROPIC_AUTH_TOKEN="sk-cp-9umbirH4beNmo7A4DsRorV2l2hJIg-XA31Vvei041QIzTn9Y5JYH6S0q-5VXfxMyqK_C7KGu3zuEavzaVtl_RKInzyZrRQzDuj2qpIH6iKvr1Q1XAv4m2qo"
$env:API_TIMEOUT_MS="3000000"
$env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC="1"
$env:ANTHROPIC_MODEL="MiniMax-M2"
$env:ANTHROPIC_SMALL_FAST_MODEL="MiniMax-M2.5"
$env:ANTHROPIC_DEFAULT_SONNET_MODEL="MiniMax-M2.5"
$env:ANTHROPIC_DEFAULT_OPUS_MODEL="MiniMax-M2.5"
$env:ANTHROPIC_DEFAULT_HAIKU_MODEL="MiniMax-M2.5"
# Iniciar Claude Code
claude --dangerously-skip-permissions .
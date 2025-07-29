import asyncio
import time
import aiohttp
import json
import matplotlib.pyplot as plt
import numpy as np
import psutil

JWT_TOKEN = "SEU_TOKEN_AQUI"
API_PROCESS_NAME = "api"

SCENARIOS = [
    {
        "name": "Leitura_GET",
        "method": "GET",
        "url": "http://127.0.0.1:8080/api/users",
        "headers": {},
        "body": None,
    },
    {
        "name": "Admin_GET_Auth",
        "method": "GET",
        "url": "http://127.0.0.1:8080/api/admin",
        "headers": {"Authorization": f"Bearer {JWT_TOKEN}"},
        "body": None,
    },
    {
        "name": "Criacao_POST",
        "method": "POST",
        "url": "http://127.0.0.1:8080/api/users",
        "headers": {"Content-Type": "application/json"},
        "body": {
            "username": "benchmark_user_py",
            "email": f"user_{int(time.time()) % 10000}@test.com",
            "password": "a_strong_password",
            "roles": ["User"]
        },
    },
]

NUM_REQUESTS = 20000
CONCURRENCY = 500

def find_process_by_name(name):
    for proc in psutil.process_iter(['pid', 'name']):
        if name.lower() in proc.info['name'].lower():
            print(f"Processo da API encontrado: PID {proc.info['pid']}, Nome: {proc.info['name']}")
            return psutil.Process(proc.info['pid'])
    return None

async def memory_monitor(process, stop_event, memory_readings):
    print("Iniciando monitor de memória...")
    while not stop_event.is_set():
        try:
            memory_mb = process.memory_info().rss / (1024 * 1024)
            memory_readings.append(memory_mb)
        except psutil.NoSuchProcess:
            print("Processo da API não encontrado. Parando monitor.")
            break
        await asyncio.sleep(0.1)
    print("Monitor de memória parado.")

def plot_memory_usage(readings, scenario_name, concurrency):
    if not readings:
        print(f"Nenhuma leitura de memória para o cenário {scenario_name}.")
        return

    peak_memory = max(readings)
    avg_memory = sum(readings) / len(readings)
    
    plt.figure(figsize=(12, 7))
    plt.plot(readings, label='Uso de Memória (RSS)', color='tab:green')
    
    plt.axhline(y=peak_memory, color='r', linestyle='--', label=f'Pico: {peak_memory:.2f} MB')
    plt.axhline(y=avg_memory, color='b', linestyle='--', label=f'Média: {avg_memory:.2f} MB')

    plt.title(f'Uso de Memória: {scenario_name}\n({concurrency} Conexões Simultâneas)', fontsize=16)
    plt.xlabel('Tempo (coletas a cada 100ms)', fontsize=12)
    plt.ylabel('Memória Usada (MB)', fontsize=12)
    plt.legend()
    plt.grid(True)
    
    filename = f"memory_usage_{scenario_name}.png"
    plt.savefig(filename)
    plt.close()
    print(f"Gráfico de memória salvo como '{filename}'!")

async def fetch(session, scenario):
    try:
        body_json = json.dumps(scenario.get("body")) if scenario.get("body") else None
        async with session.request(scenario["method"], scenario["url"], headers=scenario.get("headers"), data=body_json) as response:
            return response.status < 400
    except aiohttp.ClientError:
        return False
        
def plot_results(results, concurrency):
    print("--- Gerando gráfico de performance ---")
    names = [res["name"] for res in results]
    rps = [res["rps"] for res in results]
    latencies = [res["p90_latency"] for res in results]
    
    x = np.arange(len(names))
    width = 0.35
    
    fig, ax1 = plt.subplots(figsize=(12, 7))
    
    color_rps = 'tab:blue'
    ax1.set_xlabel('Cenário de Teste', fontsize=12)
    ax1.set_ylabel('Requisições por Segundo (RPS)', color=color_rps, fontsize=12)
    bars1 = ax1.bar(x - width/2, rps, width, label='RPS', color=color_rps)
    ax1.tick_params(axis='y', labelcolor=color_rps)
    ax1.bar_label(bars1, fmt='%.0f', padding=3)
    
    ax2 = ax1.twinx()
    color_latency = 'tab:red'
    ax2.set_ylabel('Latência p90 (ms)', color=color_latency, fontsize=12)
    bars2 = ax2.bar(x + width/2, latencies, width, label='Latência p90', color=color_latency)
    ax2.tick_params(axis='y', labelcolor=color_latency)
    ax2.bar_label(bars2, fmt='%.1f', padding=3)
    
    ax1.set_xticks(x)
    ax1.set_xticklabels(names)
    fig.suptitle(f'Benchmark de Performance da API Rust\n({concurrency} Conexões Simultâneas)', fontsize=16)
    fig.tight_layout(rect=[0, 0.03, 1, 0.95])
    
    filename = "benchmark_performance.png"
    plt.savefig(filename)
    plt.close()
    print(f"Gráfico de performance salvo como '{filename}'!")
    
async def run_scenario(scenario, api_process):
    print(f"--- Rodando cenário: {scenario['name']} ---")
    
    stop_event = asyncio.Event()
    memory_readings = []
    
    monitor_task = None
    if api_process:
        monitor_task = asyncio.create_task(memory_monitor(api_process, stop_event, memory_readings))

    tasks = []
    latencies = []
    success_count = 0
    failed_count = 0
    semaphore = asyncio.Semaphore(CONCURRENCY)
    start_time = time.time()
    
    async with aiohttp.ClientSession() as session:
        for i in range(NUM_REQUESTS):
            if scenario['method'] == 'POST' and scenario.get('body'):
                scenario['body']['email'] = f"user_{int(start_time)}_{i}@test.com"
            
            async def run_fetch():
                nonlocal success_count, failed_count
                async with semaphore:
                    req_start_time = time.time()
                    is_success = await fetch(session, scenario)
                    req_end_time = time.time()
                    if is_success:
                        success_count += 1
                        latencies.append((req_end_time - req_start_time) * 1000)
                    else:
                        failed_count += 1
            
            tasks.append(asyncio.create_task(run_fetch()))
        await asyncio.gather(*tasks)

    if monitor_task:
        stop_event.set()
        await monitor_task

    total_time = time.time() - start_time
    req_per_sec = success_count / total_time if total_time > 0 else 0
    p90 = np.percentile(latencies, 90) if latencies else 0
    
    print(f"Resultados para '{scenario['name']}': {req_per_sec:.2f} req/s, Latência p90: {p90:.2f} ms\n")
    
    plot_memory_usage(memory_readings, scenario["name"].replace(" ", "_"), CONCURRENCY)

    return { "name": scenario["name"], "rps": req_per_sec, "p90_latency": p90 }

async def main():
    api_process = find_process_by_name(API_PROCESS_NAME)
    if not api_process:
        print(f"ERRO: Não foi possível encontrar o processo com o nome '{API_PROCESS_NAME}'.")
        print("Certifique-se de que a sua API está rodando e o nome no script está correto.")
        return

    all_results = []
    for scenario in SCENARIOS:
        if "Auth" in scenario["name"] and "SEU_TOKEN_AQUI" in JWT_TOKEN:
            print(f"AVISO: Pulando cenário '{scenario['name']}'. Adicione um JWT_TOKEN válido.")
            continue
        
        result = await run_scenario(scenario, api_process)
        all_results.append(result)
        
    if all_results:
        plot_results(all_results, CONCURRENCY)
    else:
        print("Nenhum benchmark foi executado.")

if __name__ == "__main__":
    asyncio.run(main())
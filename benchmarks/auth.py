import asyncio
import time
import aiohttp
import json
import matplotlib.pyplot as plt
import numpy as np
import psutil

API_BASE_URL = "http://127.0.0.1:8080/api"
API_PROCESS_NAME = "api"

NUM_FLOWS = 2000         
CONCURRENCY = 100        
PASSWORD = "a_very_strong_password_123"

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
            break
        await asyncio.sleep(0.1)
    print("Monitor de memória parado.")

def plot_metrics(results):
    if not results:
        print("Nenhum resultado para plotar.")
        return

    plt.figure(figsize=(10, 7))
    plt.bar(['Fluxo Completo'], [results['fps']], color='tab:blue')
    plt.title(f'Benchmark de Fluxo de Usuário Completo\n({CONCURRENCY} Conexões Simultâneas)', fontsize=16)
    plt.ylabel('Fluxos por Segundo (FPS)', fontsize=12)
    plt.text(0, results['fps'], f"{results['fps']:.2f}", ha='center', va='bottom', fontsize=12)
    plt.text(0, results['fps'] / 2, f"Latência p90: {results['p90_latency']:.2f} ms", ha='center', color='white', fontsize=12)
    filename = "benchmark_full_flow_performance.png"
    plt.savefig(filename)
    plt.close()
    print(f"Gráfico de performance salvo como '{filename}'!")

    if results['memory']:
        peak_memory = max(results['memory'])
        avg_memory = sum(results['memory']) / len(results['memory'])
        plt.figure(figsize=(12, 7))
        plt.plot(results['memory'], label='Uso de Memória (RSS)', color='tab:green')
        plt.axhline(y=peak_memory, color='r', linestyle='--', label=f'Pico: {peak_memory:.2f} MB')
        plt.axhline(y=avg_memory, color='b', linestyle='--', label=f'Média: {avg_memory:.2f} MB')
        plt.title(f'Uso de Memória Durante o Benchmark de Fluxo Completo', fontsize=16)
        plt.xlabel('Tempo (coletas a cada 100ms)', fontsize=12)
        plt.ylabel('Memória Usada (MB)', fontsize=12)
        plt.legend()
        plt.grid(True)
        filename = "benchmark_full_flow_memory.png"
        plt.savefig(filename)
        plt.close()
        print(f"Gráfico de memória salvo como '{filename}'!")

async def run_full_flow(session, user_id):
    email = f"user_flow_{user_id}@test.com"
    
    create_payload = {
        "username": f"flow_user_{user_id}",
        "email": email,
        "password": PASSWORD,
        "roles": ["User"]
    }
    async with session.post(f"{API_BASE_URL}/users", json=create_payload) as response:
        if response.status >= 400:
            return False, None

    login_payload = {"email": email, "password": PASSWORD}
    async with session.post(f"{API_BASE_URL}/auth", json=login_payload) as response:
        if response.status >= 400:
            return False, None
        login_data = await response.json()
        token = login_data.get("token")
        if not token:
            return False, None

    headers = {"Authorization": f"Bearer {token}"}
    async with session.get(f"{API_BASE_URL}/me", headers=headers) as response:
        return response.status < 400, token

async def main():
    api_process = find_process_by_name(API_PROCESS_NAME)
    if not api_process:
        print(f"ERRO: Processo '{API_PROCESS_NAME}' não encontrado. A API está rodando?")
        return

    print("--- Iniciando Benchmark de Fluxo Completo ---")
    
    stop_event = asyncio.Event()
    memory_readings = []
    monitor_task = asyncio.create_task(memory_monitor(api_process, stop_event, memory_readings))

    latencies = []
    success_count = 0
    failed_count = 0
    semaphore = asyncio.Semaphore(CONCURRENCY)
    start_time = time.time()
    
    async with aiohttp.ClientSession() as session:
        tasks = []
        for i in range(NUM_FLOWS):
            async def worker(user_id):
                nonlocal success_count, failed_count
                async with semaphore:
                    flow_start_time = time.time()
                    is_success, _ = await run_full_flow(session, user_id)
                    flow_end_time = time.time()
                    
                    if is_success:
                        success_count += 1
                        latencies.append((flow_end_time - flow_start_time) * 1000)
                    else:
                        failed_count += 1
            
            tasks.append(asyncio.create_task(worker(f"{int(start_time)}_{i}")))
        
        await asyncio.gather(*tasks)

    stop_event.set()
    await monitor_task

    total_time = time.time() - start_time

    print("\n--- Resultados do Benchmark ---")
    
    fps = success_count / total_time if total_time > 0 else 0
    p90 = np.percentile(latencies, 90) if latencies else 0

    print(f"Fluxos por Segundo (FPS): {fps:.2f} fluxos/s")
    print(f"Latência p90 (por fluxo): {p90:.2f} ms")
    print(f"\nTotal de Fluxos: {NUM_FLOWS}")
    print(f"  Sucesso: {success_count}")
    print(f"  Falhas: {failed_count}")
    print(f"Tempo total: {total_time:.2f} segundos")

    plot_metrics({
        "fps": fps,
        "p90_latency": p90,
        "memory": memory_readings
    })

if __name__ == "__main__":
    asyncio.run(main())
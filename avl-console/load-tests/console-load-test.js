import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const aiResponseTime = new Trend('ai_response_time');
const vectorSearchTime = new Trend('vector_search_time');

// Test configuration
export const options = {
    stages: [
        { duration: '2m', target: 10 },   // Ramp-up to 10 users
        { duration: '5m', target: 50 },   // Ramp-up to 50 users
        { duration: '10m', target: 100 }, // Stay at 100 users
        { duration: '5m', target: 50 },   // Ramp-down to 50
        { duration: '2m', target: 0 },    // Ramp-down to 0
    ],
    thresholds: {
        http_req_duration: ['p(95)<500', 'p(99)<1000'],
        errors: ['rate<0.1'], // Error rate should be less than 10%
        ai_response_time: ['p(95)<1000'],
        vector_search_time: ['p(95)<200'],
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

// Test scenarios
export default function () {
    const scenarios = [
        testHealthCheck,
        testAIAssistant,
        testVectorSearch,
        testQuerySafety,
        testDashboard,
        testWebSocket,
    ];

    // Randomly select a scenario
    const scenario = scenarios[Math.floor(Math.random() * scenarios.length)];
    scenario();

    sleep(1);
}

function testHealthCheck() {
    const res = http.get(`${BASE_URL}/health`);

    check(res, {
        'health check status is 200': (r) => r.status === 200,
        'health check response time < 100ms': (r) => r.timings.duration < 100,
    });

    errorRate.add(res.status !== 200);
}

function testAIAssistant() {
    const queries = [
        'usuários mais ativos',
        'vendas por categoria',
        'pedidos pendentes',
        'otimização de query',
    ];

    const query = queries[Math.floor(Math.random() * queries.length)];
    const payload = JSON.stringify({ message: query });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const start = Date.now();
    const res = http.post(`${BASE_URL}/api/ai/chat`, payload, params);
    const duration = Date.now() - start;

    check(res, {
        'AI assistant status is 200': (r) => r.status === 200,
        'AI response has SQL': (r) => {
            try {
                const body = JSON.parse(r.body);
                return body.sql_query !== null;
            } catch {
                return false;
            }
        },
    });

    aiResponseTime.add(duration);
    errorRate.add(res.status !== 200);
}

function testVectorSearch() {
    const query = 'search for relevant documents';
    const payload = JSON.stringify({
        query: query,
        top_k: 5
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const start = Date.now();
    const res = http.post(`${BASE_URL}/api/vector/search`, payload, params);
    const duration = Date.now() - start;

    check(res, {
        'vector search status is 200': (r) => r.status === 200,
        'vector search response time < 200ms': (r) => r.timings.duration < 200,
    });

    vectorSearchTime.add(duration);
    errorRate.add(res.status !== 200);
}

function testQuerySafety() {
    const queries = [
        'SELECT * FROM users WHERE id = 1',
        'SELECT * FROM orders WHERE status = "pending"',
        'SELECT u.*, o.* FROM users u JOIN orders o ON u.id = o.user_id',
    ];

    const query = queries[Math.floor(Math.random() * queries.length)];
    const payload = JSON.stringify({ query: query });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const res = http.post(`${BASE_URL}/api/query/validate`, payload, params);

    check(res, {
        'query validation status is 200': (r) => r.status === 200,
        'query validation response time < 50ms': (r) => r.timings.duration < 50,
    });

    errorRate.add(res.status !== 200);
}

function testDashboard() {
    const res = http.get(`${BASE_URL}/dashboard`);

    check(res, {
        'dashboard status is 200': (r) => r.status === 200,
        'dashboard loads in < 500ms': (r) => r.timings.duration < 500,
    });

    errorRate.add(res.status !== 200);
}

function testWebSocket() {
    // Note: k6 doesn't support WebSocket in the same way as HTTP
    // This is a placeholder for WebSocket testing
    // Use a dedicated WebSocket testing tool or k6's experimental WebSocket support

    const res = http.get(`${BASE_URL}/api/stats`);

    check(res, {
        'stats endpoint status is 200': (r) => r.status === 200,
    });

    errorRate.add(res.status !== 200);
}

// Smoke test - quick validation
export function smoke() {
    const res = http.get(`${BASE_URL}/health`);
    check(res, {
        'smoke test - health check OK': (r) => r.status === 200,
    });
}

// Stress test - push to limits
export function stress() {
    for (let i = 0; i < 100; i++) {
        testAIAssistant();
        testVectorSearch();
    }
}

// Spike test - sudden load
export function spike() {
    const requests = 1000;
    const responses = http.batch([
        ['GET', `${BASE_URL}/health`],
        ['GET', `${BASE_URL}/dashboard`],
        ['GET', `${BASE_URL}/api/stats`],
    ].slice(0, requests));

    responses.forEach((res) => {
        check(res, {
            'spike test - status is 200': (r) => r.status === 200,
        });
    });
}

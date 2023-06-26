"use client";
// import React from 'react';
// import Chart from 'chart.js/auto';
// import { Line } from 'react-chartjs-2';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// import 'chartjs-adapter-date-fns';
// import { ja } from 'date-fns/locale';
// import { Box } from '../common/components';

import React from 'react';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';
import StreamingPlugin from 'chartjs-plugin-streaming';
import 'chartjs-adapter-date-fns';
import { ja } from 'date-fns/locale';
import { Box } from '../common/components';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  StreamingPlugin
);

export function LineChart() {
  return (
    <Box>
      <Line
        data={{
          datasets: [
            {
              label: 'CPU',
              borderColor: '#abcde8',
              backgroundColor: '#abcde8',
              data: [],
            },
            {
              label: 'MEM',
              borderColor: '#c7def1',
              backgroundColor: '#c7def1',
              data: [],
            },
            {
              label: 'DISK',
              borderColor: '#c4b7f7',
              backgroundColor: '#c4b7f7',
              data: [],
            },
            {
              label: 'NETWORK',
              borderColor: '#91D8E4',
              backgroundColor: '#91D8E4',
              data: [],
            },
          ],
        }}
        options={{
          plugins: {
            title: {
              display: true,
              text: 'リソース利用状況'
            }
          },
          scales: {
            x: {
              type: 'realtime',
              title: {
                display: true,
                text: '時間(JST)',
              },
              adapters: {
                date: {
                  locale: ja,
                },
              },
              time: {
                unit: 'second',
              },
              realtime: {
                duration: 30000,
                delay: 2000,
                refresh: 2000,
                pause: false,
                ttl: undefined,
                onRefresh: (chart) => {
                  const now = Date.now()
                  chart.data.datasets.forEach((dataset) => {
                    dataset.data.push({
                      x: now,
                      y: Math.floor(Math.random() * 101),
                    })
                  })
                },
              },
            },
            y: {
              title: {
                display: true,
                text: '使用率(%)',
              },
              min: 0,
              max: 100,
            },
          },
        }}
      />
    </Box>
  )
}
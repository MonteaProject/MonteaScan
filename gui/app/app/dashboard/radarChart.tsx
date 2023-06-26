"use client";
// import React from 'react';
// import Chart from 'chart.js/auto';
// import { Radar } from 'react-chartjs-2';
// import StreamingPlugin from 'chartjs-plugin-streaming';
// Chart.register(StreamingPlugin);
// import 'chartjs-adapter-date-fns';
// import { ja } from 'date-fns/locale';
// import { Box } from '../common/components';

import React from 'react';
import {
  Chart as ChartJS,
  RadialLinearScale,
  PointElement,
  LineElement,
  Filler,
  Tooltip,
  Legend,
} from 'chart.js';
import { Radar } from 'react-chartjs-2';
import { Box } from '../common/components';

ChartJS.register(
  RadialLinearScale,
  PointElement,
  LineElement,
  Filler,
  Tooltip,
  Legend
);

export function RadarChart() {
  return (
    <Box>
      <Radar
        data={{
          labels: ['Thing 1', 'Thing 2', 'Thing 3', 'Thing 4', 'Thing 5', 'Thing 6'],
          datasets: [
            {
              label: '# of Votes',
              data: [2, 9, 3, 5, 2, 3],
              backgroundColor: 'rgba(255, 99, 132, 0.2)',
              borderColor: 'rgba(255, 99, 132, 1)',
              borderWidth: 1,
            },
          ]
        }}
      />
    </Box>
  )
}
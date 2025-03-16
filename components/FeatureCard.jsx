// components/FeatureCard.js
import React from 'react';
import { motion } from 'framer-motion';

const FeatureCard = ({ title, description, icon, delay = 0 }) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.6, delay }}
      viewport={{ once: true }}
      className="bg-[#0a0a2a] p-6 rounded-xl shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-purple-800 hover:border-purple-600 h-full flex flex-col"
    >
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-bold mb-3 text-white">{title}</h3>
      <p className="text-purple-300 flex-grow">{description}</p>
      <div className="mt-6">
        <a href="#" className="text-blue-400 hover:text-blue-300 flex items-center">
          Learn more
          <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </a>
      </div>
    </motion.div>
  );
};

export default FeatureCard;
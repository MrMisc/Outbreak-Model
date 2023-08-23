import cv2
import json
from matplotlib import rc
from matplotlib.font_manager import FontProperties
from scipy.stats import ks_2samp
import scipy.stats as stats  
import os
import seaborn as sns
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import plotly.express as px
import subprocess
percentage_h,total_h,no_h,percentage_d,total_d,no_d = ([] for _ in range(6))
while True:
    try:
        # The last set of DAT, is actually index information about the simulation parameters
        # Removing and storing that valueset from these lists would be ideal
        #The index format for the last element in DAT is method number (ends up inside the data lists) - max number of runs per attempt(tapdata lists) - falsecap(money lists)
        DAT= input().split(" ")
        percentage_h.append(float(DAT[0]))
        total_h.append(int(DAT[1]))
        no_h.append(int(float(DAT[2])))
        percentage_d.append(float(DAT[3]))
        total_d.append(int(DAT[4]))
        no_d.append(int(float(DAT[5])))
    except EOFError as e:
        break


#Resorting the data
result_percentage_h = percentage_h[-1]
result_total_h = total_h[-1]
result_no_h = no_h[-1]
result_percentage_d = percentage_d[-1]
result_total_d = total_d[-1]
result_no_d = no_d[-1]


#remove the last element
percentage_h = percentage_h[:-1]
total_h = total_h[:-1]
no_h = no_h[:-1]
percentage_d = percentage_d[:-1]
total_d = total_d[:-1]
no_d = no_d[:-1]

# print(percentage_h)


#Percentage of hosts
plt.clf()
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
hours = list(range(1,len(percentage_h)+1))
fig = px.line(x = hours,y = [percentage_h,percentage_d],title = "Percentage of Hosts that are infected by hour", line_dash_sequence=['solid', 'dot'])

fig.update_layout(
    xaxis_title='Hour',
    yaxis_title='Percentage'
)

fig.update_layout(
    xaxis_title_font=dict(family="Yu Gothic"),
    yaxis_title_font=dict(family="Yu Gothic"),
    font=dict(family="Yu Gothic"),
    plot_bgcolor='#fff6eb'
)

fig.show()




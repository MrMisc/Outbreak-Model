#! /usr/bin/Rscript

#! /usr/bin/Rscript

f<-file("stdin")
open(f)
record<-c()
while(length(line<-readLines(f,n=1))>0){
  #write(line,stderr())
  record<-c(record,line)  
}


numbers<-record[1:length(record)-1]
numbers_<-c()


print(getwd())


coordinates <- strsplit(record, " ")

# Extract x, y, and time values
x <- as.numeric(sapply(coordinates, "[[", 1))
y <- as.numeric(sapply(coordinates, "[[", 2))
time <- as.numeric(sapply(coordinates, "[[", 3))

# Plot heatmap
library(ggplot2)
library(pandoc)

# Extract x, y coordinates
coordinates <- strsplit(record, " ")

# Extract x, y, and time values
x <- as.numeric(sapply(coordinates, "[[", 1))
y <- as.numeric(sapply(coordinates, "[[", 2))
time <- as.numeric(sapply(coordinates, "[[", 3))

# Plot heatmap
library(ggplot2)
library(plotly)

data <- data.frame(x = x, y = y)
heatmap_plot <- ggplot(data, aes(x, y)) +
  geom_bin2d(bins = 20) +  # Adjust bins as needed
  scale_fill_viridis_c() +  # You can choose other color scales too
  labs(title = "Heatmap of Coordinates")

# Convert ggplot to ggplotly
heatmap_interactive <- ggplotly(heatmap_plot)

# Save as HTML using pandoc
htmlwidgets::saveWidget(heatmap_interactive, "heatmap_output.html", selfcontained = TRUE)
print("Heatmap generated successfully!")




#Finding all files that contain this name are as follows
data<-read.csv("output.csv",header = FALSE)

library("ggplot2")
library("plotly")
library("breakDown")
library(ggdark)
library(pracma)
library(comprehenr)
library(ggridges)
library(tidyverse)
library(ggplot2)
library(plotly)
library(thematic)
library(extrafont)
library(pandoc)
#library(pandoc)
#Get dem custom fonts
# font_import()
# loadfonts(device = "win")
# actual_pars<-as.data.frame(actual_pars)



colnames(data) <- c(
  "HitPct1", "TotalSamples1", "HitSamples1",
  "HitPct2", "TotalSamples2", "HitSamples2",
  "HitPct3", "TotalSamples3", "HitSamples3",
  "HitPct4", "TotalSamples4", "HitSamples4"
)

# Scatter plot for the first 2 sets of data
# Define custom theme colors
thematic_on(bg = "#FCE9D7", fg = "orange", accent = "purple",font = "Yu Gothic")

time <- seq_len(nrow(data))
# Scatter plot for the first 2 sets of data
# scatter_plot_1 <- ggplot(data) +
#   geom_point(aes(x = time, y = HitPct1), fill = "#007663") +geom_point(aes(x = time, y = HitPct2), fill = "#FF8A5F") +
#   labs(title = "Scatter Plot: First 2 Sets") +
#   theme_bw() +  # Applying the custom theme
#   theme(
#     legend.background = element_rect(
#       fill = "#FCE9D7",
#       colour = "black",
#       size = 1
#     )
#   )



# fig_dots<-data%>%
#   plot_ly(x = time,
#           y = ~HitPct1,
#           color ="Host",
#           colors=c("#00927D","#00C9B1"),
#           size = ~TotalSamples1,
#           customdata = ~HitSamples1,
#           hovertemplate="%{y} % of motile hosts <br> are infected  <br> ie %{customdata} out of %{marker.size} hosts",
#           type="scatter",
#           mode = "markers+lines",line = list(width=0.35)) 

# fig_dots<-fig_dots %>%
#   add_trace(
#     x = ~time,
#     y = ~HitPct2,
#     color = "Deposits",
#     colors = c("#F7C9B6", "#FF8371"),  # Reversed color order
#     size = ~TotalSamples2,
#     customdata = ~HitSamples2,
#     hovertemplate = "%{y} % of motile hosts <br> are infected  <br> ie %{customdata} out of %{marker.size} hosts",
#     line = list(width = 0.35)
#   ) %>%
#   layout(title = "% of marriages that end in divorce",
#          plot_bgcolor = '#FFF8EE',
#          xaxis = list(
#           title = "Time (Hours)",
#            zerolinecolor = '#ffff',
#            zerolinewidth = 0.5,
#            gridcolor = '#F4F2F0'),
#          yaxis = list(
#           title = "Percentage of Infected",
#            zerolinecolor = '#ffff',
#            zerolinewidth = 0.5,
#            gridcolor = '#F4F2F0'))




#Farm

fig_dots<-data%>%plot_ly(type="scatter",
          mode = "markers+lines",line = list(width=0.35))%>%
  add_trace(x = time,
          y = ~HitPct1,
          color ="Host",
          colors=c("#2A6074","#00C9B1"),
          size = ~TotalSamples1,
          customdata = ~paste(HitSamples1, "out of ", TotalSamples1," hosts"),
          hovertemplate="%{y} % of motile hosts <br> are infected  <br> ie %{customdata}")


fig_dots<-fig_dots %>%
  add_trace(
    x = ~time,
    y = ~HitPct2,
    color = "Deposits",
    colors = c("#FFF184", "#FFDD80"),  # Reversed color order
    size = ~TotalSamples2,
    customdata = ~paste(HitSamples2, "out of ", TotalSamples2," deposits"),
    hovertemplate = "%{y} % of sessile deposits <br> are infected  <br> ie %{customdata}",
    line = list(width = 0.35)
  ) %>%
  layout(title = "Infection Trend within cultivation",
         plot_bgcolor = '#FFF8EE',
         xaxis = list(
          title = "Time (Hours)",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'),
         yaxis = list(
          title = "Percentage of Infected",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'))

htmlwidgets::saveWidget(fig_dots, "scatter_plot_1.html", selfcontained = TRUE)



#Collection

fig_dots<-data%>%plot_ly(type="scatter",
          mode = "markers+lines",line = list(width=0.35))%>%
  add_trace(x = time,
          y = ~HitPct3,
          color ="Host",
          colors=c("#2A6074","#00C9B1"),
          size = ~TotalSamples3,
          customdata = ~paste(HitSamples3, "out of ", TotalSamples3," hosts"),
          hovertemplate="%{y} % of motile hosts <br> are infected  <br> ie %{customdata}")


fig_dots<-fig_dots %>%
  add_trace(
    x = ~time,
    y = ~HitPct4,
    color = "Deposits",
    colors = c("#FFF184", "#FFDD80"),  # Reversed color order
    size = ~TotalSamples4,
    customdata = ~paste(HitSamples4, "out of ", TotalSamples4," deposits"),
    hovertemplate = "%{y} % of sessile deposits <br> are infected  <br> ie %{customdata}",
    line = list(width = 0.35)
  ) %>%
  layout(title = "Infection Trend within collection",
         plot_bgcolor = '#FFF8EE',
         xaxis = list(
          title = "Time (Hours)",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'),
         yaxis = list(
          title = "Percentage of Infected",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'))


htmlwidgets::saveWidget(fig_dots, "scatter_plot_2.html", selfcontained = TRUE)
